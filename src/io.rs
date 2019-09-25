/*******************************************************************************
Title           : io.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19
*******************************************************************************/
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use crate::error::MyError;
use std::process::Command;


pub struct Files{ fields: File, types: File }
#[derive(Debug, Clone)]
pub struct Lines{ fields: String, types:String }
#[derive(Debug, Clone)]
pub struct Values<'a>{ fields: Vec<&'a str>, types: Vec<&'a str> }
#[derive(Debug, Clone)]
pub struct Commands{ user: String, pass: String }

#[allow(dead_code)]
pub enum Io<'a>{
    EFiles(Files),
    ELines(Lines),
    EValues(Values<'a>),
}

pub fn get_values(lines: &Lines) -> Values {
       Values{
           fields: lines.fields.split(',').collect(),
           types: lines.types.split(',').collect()
       }
}

pub fn check_fields(values:Values) -> Result<(), Box<dyn Error>> {
    if values.fields.len() != values.types.len() {
        eprintln!("invalid information");
        MyError::field_error();
    }
    Ok(())
}

pub fn get_lines(files:Files) -> Result<Lines,
    Box<dyn Error>>{
    let fields = get_first_line(files.fields)?;
    let types = get_first_line(files.types)?;
    Ok( Lines{fields,types} )
}

pub fn get_first_line(file: File) -> Result<String, std::io::Error> {
    let val= BufReader::new(file).lines().nth(0).unwrap();
    Ok(val?)
}

pub fn check_file(data_file: &str, type_file: &str) -> Result<Files,
    Box<dyn Error>> {
    let fields=File::open(Path::new(data_file))?;
    let types=File::open(Path::new(type_file))?;
    Ok(Files{ fields, types })
}

pub fn get_query(mut query: String, values: Values, table_name: &str)
    -> String {
    let mut tmp: String = format!("create table if not exists {}(", table_name);
    let mut it = values.types.iter().peekable();
    
    query.push_str(tmp.as_str());
    for pair in values.fields.iter().map(|x| (x, it.next())) {
        tmp = format!("{} {},", pair.0, pair.1.unwrap());
        query.push_str(tmp.as_str());
    }
    query.pop();
    query.push(')');
    query
}

pub fn get_commands(user:String, pass:String)->Commands{
    Commands{
        user: format!("--user={}", user),
        pass :format!("--password={}", pass)
    }
}

pub fn insert(commands:Commands, data_base: &str, file_name: &str) {
    Command::new("mysqlimport")
    .arg("--ignore-lines=1")
    .arg("--fields-terminated-by=:")
    .arg("--local")
    .arg(commands.user)
    .arg(commands.pass)
    .arg(data_base)
    .arg(file_name)
    .status()
    .expect("failed to execute process");
}
