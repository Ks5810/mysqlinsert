/*******************************************************************************
Title           : mysqlinsert.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19
Description     : A practice rust app that enables creating a table, inserting
                  a csv formatted file directly from command line. This app
                  will create a name as the file without .extension. Tests are
                  not written yet. For more details, please have a look at
                  README.md
Modification    : Made on 10/2/19
                  Added options for field separator and line terminator.
*******************************************************************************/
#[macro_use]
extern crate clap;
extern crate mysqlinsert;

use std::process;
use mysqlinsert::*;

fn main() {
    //using clap
    let matches = clap_app!(mysqlinsert =>
            (version: "1.0")
            (author: "Keisuke Suzuki <e40keisuke@gmail.com>")
            (about: "Reads file and inserts data to MySQL")
            (@arg FIELD_SEPARATOR: -f --field +takes_value
                            "Sets a filed separator. Default value is `,` ")
            (@arg LINE_TERMINATOR: -t --line +takes_value
                            r"Sets a terminator. Default value is `\n` ")
            (@arg FIELD_FILE: +required "Sets an input file for fields")
            (@arg TYPE_FILE: +required "Sets a input file for types"))
        .get_matches();

    // set separator and terminator to entered value, if options are not
    // selected, set them to ',', '\n' respectively
    let separator = matches.value_of("FIELD_SEPARATOR").unwrap_or(",");
    let terminator = matches.value_of("LINE_TERMINATOR").unwrap_or(r"\n");

    // get input files from arguments. if either or both values are empty,
    // displays usage
    let f_file = matches.value_of("FIELD_FILE").unwrap();
    let t_file = matches.value_of("TYPE_FILE").unwrap();

    // it insert_files return an error, it displays the details
    if let Err(e) = insert_files(f_file, t_file, separator, terminator){
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}