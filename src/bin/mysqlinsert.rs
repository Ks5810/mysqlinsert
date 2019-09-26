/*******************************************************************************
Title           : mysqlinsert.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19
Description     : A practice rust app that enables creating a table, inserting
                  a csv formatted file directly from command line. This app
                  will create a name as the file without .extension. Tests are
                  not written yet. For more details, please have a look at
                  README.md
*******************************************************************************/
extern crate clap;
extern crate mysqlinsert;

use clap::{App, Arg};

fn main() {
    let matches = App::new("mysqlinsert")
    .version("1.0")
    .author("Keisuke Suzuki <e40keisuke@gmail.com>")
    .usage(
        "mysqlinsert <path to data file> <path \
             to type file>")
    .args(&[
        Arg::with_name("data_file")
        .required(true)
        .index(1)
        .help("file with comma separated field names")
        .value_name("data_file"),
        Arg::with_name("type_file")
        .required(true)
        .index(2)
        .help("file with comma separated typename names")
        .value_name("type_file")]
    )
    .get_matches();
    
    let data=matches.is_present("data_file");
    let types=matches.is_present("type_file");
    if  !(data&&types) {
        println!("{}", matches.usage())
        }
    else {
        let data_file = matches.value_of("data_file").unwrap();
        let type_file = matches.value_of("type_file").unwrap();
        if let Err(e) = mysqlinsert::insert_files(data_file, type_file) {
            println!("error {}", e);
        }
    }
}