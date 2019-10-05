/*******************************************************************************
Title           : lib.rs
Author          : Keisuke Suzuki
Created on      : Sep 17, 2019
Description     : lib.rs for practice app

*******************************************************************************/
#[macro_use]
extern crate command_macros;
extern crate csv;

pub mod error;
pub mod my;
pub mod args;

pub fn insert_files(f_file: &str, t_file: &str, sep: &str, ter: &str)
                    -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::new(f_file, t_file, sep, ter);
    let my=my::MysqlInsert::new()?;
    let query = args::get_query(args)?;
    my.prep_exec(query)?;
    my.insert(f_file,sep,ter)?;
    Ok(())
}
