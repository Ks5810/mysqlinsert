/*******************************************************************************
Title           : error.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19
Modification    :
*******************************************************************************/
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MyError {
    message: String,
}

#[allow(dead_code)]
impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{message: msg.to_string()}
    }
    
    pub fn field_error() -> MyError {
        MyError {
            message: String::from("Invalid field information"),
        }
    }
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