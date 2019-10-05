/*******************************************************************************
Title           : args.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19

*******************************************************************************/
extern crate itertools;
use std::error::Error as StdError;
use super::error::Error;
use std::fs::File;
use std::io::{BufReader,BufRead};

type BoxResult<T> = Result<T, Box<dyn StdError>>;

pub struct FPair(File, File);
pub struct SPair(String, String);
pub struct Vecs<'a>(Vec<&'a str>, Vec <&'a str>);

// helper functions

fn split<'a>(target: &'a str, sep: &str) -> Vec<&'a str> {
    target.split(sep).collect()
}
fn split_file_name(target: &str) -> Vec<&str> {
    target.split(|x| x=='/'||x=='.').collect()
}
fn remove_tabspace(s: &mut String) {
    s.replace('\t',"");
}
fn remove_whitespace(s:&mut String) {
    s.retain(|c| c.is_whitespace())
}
fn get_ter(tar:&str) -> u8{
    match tar {
        r"\n" => b'\n',
        r"\r" => b'\r',
        _ => b'\n'
    }
}
fn get_sep(sep: &str) -> u8{
    if sep == r"\t" { b'\t'}
    else {0}
}
fn get_last(words: &[&str]) -> String {
    words.last().unwrap().chars().last().unwrap().to_string()
}
fn same_len(lhs: &[&str], rhs: &[&str]) -> bool{
    lhs.len()==rhs.len()
}
fn is_alphanum(val: &str) -> bool {
    let c = val.chars().next().unwrap();
    c.is_alphanumeric()
}
fn carriage_return(string: &str) -> bool {
    string.find('\r').is_some()
}
fn valid_multi_tar(st: &str) -> bool{
    st == r"\n"|| st == r"\r"
}

fn modify_string(string: &str) -> String{
    if carriage_return(string) {
        let tmp: Vec<_> = string.split('\r').collect();
        let tmp = tmp[0].clone();
        tmp.to_string()
    }
    else {string.to_string()}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Args<'a> {
    f_file: &'a str,
    t_file: &'a str,
    sep: &'a str,
    ter: &'a str,
}

use crate::error::ErrorKind::*;
use itertools::Itertools;
use std::path::Path;

impl<'a> Args<'a> {
    // constructor
    pub fn new(f_file: &'a str, t_file: &'a str,
        sep: &'a str, ter: &'a str) -> Args<'a> {
        Args { f_file, t_file, sep, ter}
    }
    pub fn get_files(&self) -> BoxResult<FPair>{
        Ok(FPair(File::open(Path::new(self.f_file))?,
              File::open(Path::new(self.t_file))?))
    }
    pub fn get_lines(&self, files: FPair) -> BoxResult<SPair> {
        Ok(SPair(self.get_first_line(files.0)?,
                 self.get_first_line(files.1)?))
    }


    // gets first line from both filed and type files
    fn get_first_line(&self, file: File) -> BoxResult<String> {
        let mut string=String::new();
        let val = BufReader::new(file).read_line(&mut string)?;
        let ter=self.ter.chars().nth(0).unwrap();
        let res = modify_string(&string);
        Ok(res)
    }
    // checks fields and returns error if invalid
    pub fn check_fields(&self, vecs: &Vecs) -> BoxResult<()> {
        if !same_len(&vecs.0,&vecs.1) {
            return Err(Box::new(Error::kind(MatchCount)))
        } else if !valid_multi_tar(self.ter) && get_last(&vecs.0) != self.ter {
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
        } else if !valid_multi_tar(&self.ter) {
            return Err(Box::new(Error::kind(MultipleTer)))
        } Ok(())
    }
    // gets table name from input file path
    pub fn get_table_name(&self) -> BoxResult<&str> {
        let mut vec: Vec<_> = split_file_name(self.f_file);
        vec.pop().ok_or(Error::kind(IoError))?;
        Ok(vec.last().ok_or(Error::kind(IoError))?)
    }

    pub fn get_query(&self, lines: SPair) -> BoxResult<String> {
        let vecs = Vecs(split(&lines.0,self.sep),
                        split(&lines.1,self.sep));
        println!("vecs0: {:?}, vecs1: {:?}",vecs.0,vecs.1);
        self.check_fields(&vecs)?;
        let query: Vec<_> = vecs.0.iter().zip(vecs.1.iter())
                                .map(|(f, t)| format!("{} {}", *f, *t)).collect();
        Ok(format!("create table if not exists {}({})", self.get_table_name()?,
                   query.iter().join(", ")))
    }
    //prints information
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
    let files = args.get_files()?;
    let lines = args.get_lines(files)?;
    let query = args.get_query(lines)?;
    Ok(query)
}


