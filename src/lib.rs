/*******************************************************************************
Title           : lib.rs
Author          : Keisuke Suzuki
Created on      : Sep 17, 2019
Description     : lib.rs for practice app
localhost
Usage           :
*******************************************************************************/
pub mod error;
pub mod my;
pub mod example;
pub mod io;

use std::error::Error;
use io::*;

pub fn get_vec(file_name:&str)->Vec<&str> {
    file_name.split(|x| x == '/' || x == '.').collect()
}

pub fn get_filename(file_name:&str)->&str{
    let mut vec=get_vec(file_name).clone();
    vec.pop();
    vec.last().unwrap()
}

pub fn insert_examples() -> Result <(), Box<dyn Error>>{
    unimplemented!();
}

pub fn insert_files(data_file: &str, type_file: &str) -> Result<(), Box<dyn Error>> {
    let my=my::Mysql::new()?;
    let file_name = data_file;
    let files: Files = check_file(data_file, type_file)?;
    let lines: Lines = get_lines(files)?;
    let values: Values = get_values(&lines);
    check_fields(values.clone()).unwrap();
    let table_name = get_filename(&file_name);
    let query = String::new();
    my.prep_exec(get_query(query,values, table_name))?;
    let commands=get_commands(my.env.user().to_string(),
                              my.env.password().to_string());
    insert(commands, my.env.database(), file_name);
    Ok(())
}
