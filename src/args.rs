/*******************************************************************************
Title           : args.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19

*******************************************************************************/
extern crate itertools;

use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use itertools::Itertools;

type MyE = crate::error::MyError;
type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct Files(File, File);
pub struct Pair(String, String);
pub struct Words<'a>(Vec<&'a str>, Vec <&'a str>);

// helper functions

fn split<'a>(target: &'a str, sep: &str) -> Vec<&'a str> {
    target.split(sep).collect()
}
fn split_file_name(target: &str) -> Vec<&str> {
    target.split(|x| x=='/'||x=='.').collect()
}
fn get_last_char(word: &Vec<&str>) -> String {
    word.last().unwrap().chars().last().unwrap().to_string()
}


#[derive(Clone)]
pub struct Args<'a> {
    f_file: &'a str,
    t_file: &'a str,
    sep: &'a str,
    ter: &'a str,
}

impl<'a> Args<'a> {
    // constructor
    pub fn new(f_f: &'a str, t_f: &'a str,
        sep: &'a str, ter: &'a str) -> Args<'a> {
        Args { f_file: f_f, t_file: t_f, sep, ter }
    }
    // checks file and returns error if unable to open file(s)
    pub fn check_files(&self) -> BoxResult<Files> {
        let fields = File::open(Path::new(self.f_file))?;
        let types = File::open(Path::new(self.t_file))?;
        Ok(Files(fields, types))
    }
    // checks fields and returns error if invalid
    pub fn check_fields(&self, values: &Words) -> BoxResult<()> {
        if values.0.len() != values.1.len() {
            MyE::e("field");
        } else if self.ter != "\n" {
            if get_last_char(&values.0) != self.ter{ MyE::e("terfi"); }
        }
        Ok(())
    }
    // checks field separator and throws error if invalid
    pub fn check_separator(&self) -> BoxResult<&str> {
        if self.sep.len() == 1 {
            if Self::check_alphanum(self.sep) { MyE::e("alpha"); }
        } else if self.sep != "\t" { MyE::e("multi"); }
        Ok(self.sep)
    }
    // checks line terminator and throws error if invalid
    pub fn check_terminator(&self) -> BoxResult<&str> {
        if self.ter.len() == 1 {
            if Self::check_alphanum(self.ter) { MyE::e("alpha"); }
        } else if self.ter != "\n" { MyE::e("multi"); }
        Ok(self.ter)
    }
    // checks if alphanumeric value is not used for separator or terminator
    fn check_alphanum(val: &str) -> bool {
        let vec: Vec<_> = val.chars().collect();
        let c = vec.first().unwrap();
        c.is_alphanumeric()
    }
    // gets table name from input file path
    pub fn get_table_name(&self) -> BoxResult<&str> {
        let mut vec: Vec<_> = split_file_name(self.f_file);
        vec.pop();
        let v: &str = vec.last().ok_or(MyE::e("file"))?;
        Ok(v)
    }
    // gets first line from both filed and type files
    fn get_first_line(&self, file: File) -> BoxResult<String> {
        let val = BufReader::new(file).lines().nth(0).ok_or(MyE::e("error"))?;
        Ok(val?)
    }
    // gets lines as Lines
    pub fn get_lines(&self, files: Files) -> BoxResult<Pair> {
        let fields = self.get_first_line(files.0)?;
        let types = self.get_first_line(files.1)?;
        Ok(Pair(fields, types))
    }
    // create query from field and type info in Pair, and return query as string
    pub fn get_query(&self, lines: Pair) -> BoxResult<String> {
        let words=Words(
            split(&lines.0, self.sep),
            split(&lines.1, self.sep)
        );
        self.check_fields(&words)?;
        let query: Vec<_> = words.0.iter().zip(words.1.iter())
                                .map(|(f, t)| format!("{} {}", *f, *t)).collect();
        Ok(format!("create table if not exists {}({})", self.get_table_name()?,
                   query.iter().join(", ")))
    }
}

// for parent module. returns query for 'create table'
pub fn get_query(args: Args) -> BoxResult<String> {
    let files = args.check_files()?;
    let lines = args.get_lines(files)?;
    let query = args.get_query(lines)?;
    Ok(query)
}


