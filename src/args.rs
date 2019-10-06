/*******************************************************************************
Title           : args.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19

*******************************************************************************/
extern crate itertools;

use itertools::Itertools;
use std::error::Error as StdError;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader,BufRead};
use super::error::{Error,ErrorKind::*};

type BoxResult<T> = Result<T, Box<dyn StdError>>;


// helper functions

fn split<'a>(target: &'a str, sep: &str) -> Vec<&'a str> {
    target.split(sep).collect()
}
fn split_file_name(target: &str) -> Vec<&str> {
    target.split(|x| x == '/' || x == '.').collect()
}
fn remove_tab(s: &mut String) {
    s.replace('\t', "");
}
fn remove_whitespace(s: &mut String) {
    s.retain(|c| c.is_whitespace())
}
fn get_ter(s:&str) -> char{
    match s {
        r"\n" => '\n',
        r"\r" => '\r',
        _ => '\n'
    }
}
fn get_sep(s: &str) -> char{
    if s == r"\t" { '\t'}
    else { get_first_char(s) }
}
fn get_last_char(words: &[&str]) -> String {
    words.last().unwrap().chars().last().unwrap().to_string()
}
fn get_first_char(s: &str) -> char{
    s.chars().nth(0).unwrap()
}
fn same_len(lhs: &[&str], rhs: &[&str]) -> bool{
    lhs.len() == rhs.len()
}
fn is_alphanum(s: &str) -> bool {
    let c = s.chars().next().unwrap();
    c.is_alphanumeric()
}
fn contains_carriage_return(s: &str) -> bool {
    s.find('\r').is_some()
}
fn valid_tar(s: &str) -> bool{
    s == r"\n"|| s == r"\r"
}
fn check_carriage_return(s: &str) -> String {
    if contains_carriage_return(s) {
        let tmp: Vec<_> = s.split('\r').collect();
        tmp[0].clone().to_string()
    }
    else { s.to_string() }
}

pub struct FPair(File, File);
pub struct SPair(String, String);
pub struct VPair<'a>(Vec<&'a str>, Vec <&'a str>);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Args<'a> {
    f_file: &'a str,
    t_file: &'a str,
    sep: &'a str,
    ter: &'a str,
}

impl<'a> Args<'a> {
    // constructor
    pub fn new(f_file: &'a str, t_file: &'a str,
        sep: &'a str, ter: &'a str) -> Args<'a> {
        Args { f_file, t_file, sep, ter}
    }
    // checks fields and returns error if invalid
    pub fn check_fields(&self, vecs: &VPair) -> BoxResult<()> {
        if !same_len(&vecs.0,&vecs.1) {
            return Err(Box::new(Error::kind(MatchCount)))
        } else if !valid_tar(self.ter) && get_last_char(&vecs.0) != self.ter {
            return Err(Box::new(Error::kind(MatchTer)))
        }
        Ok(())
    }
    // checks field separator and throws error if invalid
    pub fn check_separator(&self) -> BoxResult<()> {
        if self.sep.len() == 1 { if is_alphanum(&self.sep) {
            return Err(Box::new(Error::kind(AlphanumSep))) }
        } else if self.sep != r"\t" {
            return Err(Box::new(Error::kind(MultipleSep)))
        } Ok(())
    }
    // checks line terminator and throws error if invalid
    pub fn check_terminator(&self) -> BoxResult<()> {
        if self.ter.len() == 1 { if is_alphanum(&self.ter) {
           return Err(Box::new(Error::kind(AlphanumTer))) }
        } else if !valid_tar(&self.ter) {
            return Err(Box::new(Error::kind(MultipleTer)))
        } Ok(())
    }
    // gets table name from input file path
    pub fn get_table_name(&self) -> BoxResult<&str> {
        let mut vec: Vec<_> = split_file_name(self.f_file);
        vec.pop().ok_or(Error::kind(IoError))?;
        Ok(vec.last().ok_or(Error::kind(IoError))?)
    }
    // gets pair of files if successfully opened
    pub fn get_file_pair(&self) -> BoxResult<FPair>{
        Ok(FPair(File::open(Path::new(self.f_file))?,
                 File::open(Path::new(self.t_file))?))
    }
    // get pair of strings that contains first lines of files
    pub fn get_line_pair(&self, files: FPair) -> BoxResult<SPair> {
        Ok(SPair(self.get_first_line(files.0)?,
                 self.get_first_line(files.1)?))
    }
    // gets first line from both filed and type files
    fn get_first_line(&self, file: File) -> BoxResult<String> {
        let mut line=String::new();
        let _len = BufReader::new(file).read_line(&mut line)?;
        //if carriage return found, get line
        line = check_carriage_return(&line);
        Ok(line)
    }
    // get query string from lines
    pub fn get_query(&self, lines: SPair) -> BoxResult<String> {
        let vecs = VPair(split(&lines.0, self.sep),
                         split(&lines.1,self.sep));
        self.check_fields(&vecs)?;
        let query: Vec<_> = vecs.0.iter().zip(vecs.1.iter())
                                .map(|(f, t)| format!("{} {}", *f, *t)).collect();
        Ok(format!("create table if not exists {}({})", self.get_table_name()?,
                   query.iter().join(", ")))
    }
    // prints information
    pub fn print_info(&self){
        println!("\tData File: {}\n\
                  \tType File: {}\n\
                  \tField Separator: `{}`\n\
                  \tLine Terminator: `{}`",
                  self.f_file, self.t_file, self.sep, self.ter);
    }
}

// for parent module. returns query for 'create table'
pub fn get_query(args: Args) -> BoxResult<String> {
    args.print_info();
    args.check_separator()?;
    args.check_terminator()?;
    let files = args.get_file_pair()?;
    let lines = args.get_line_pair(files)?;
    let query = args.get_query(lines)?;
    Ok(query)
}


