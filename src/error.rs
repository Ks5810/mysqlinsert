/*******************************************************************************
Title           : error.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19
Modification    :
*******************************************************************************/
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MyError{
    message: String
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.message)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
            &self.message
        }
    }

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError { message: msg.to_string() }
    }
    pub fn e(info: &str) -> MyError {
        let message = match info {
            "alpha" => "Alphanumeric value is selected for separator, \
                           please change it to other values",
            "multi" => "Separator cannot be more than one character, except \
                           for tab space '/t'",
            "field" => "number of fields and types do not match. Please check \
                        those files",
            "ter" => "Invalid terminator",
            "file" => "Invalid file name",
            "terfi" => "Terminator in files does not match to selected one",
            "empty" => "last character is empty",
            _ => "Unknown Error",
        };
        MyError { message: format!("Error: {}", message) }
    }
}