/*******************************************************************************
Title           : error.rs
Author          : Keisuke Suzuki
Created on      : 9/25/19
Modification    :
*******************************************************************************/
use std::error::Error as StdError;
use std::fmt;

pub enum ErrorKind {
    IoError,
    MatchCount,
    MatchTer,
    AlphanumSep,
    AlphanumTer,
    MultipleSep,
    MultipleTer,
    EmptyChar,
}

#[derive(Debug)]
pub struct Error {
    message: String
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", &self.message)
    }
}
impl StdError for Error {
    fn description(&self) -> &str {
        &self.message
    }
}
impl Error {
    pub fn kind(kind: ErrorKind) -> Error {
        use ErrorKind::*;
        let message = match kind {
            IoError     => "an I/O error occurred",
            MatchCount  => "Length of fields and types do not match. Please \
                            check those files",
            MatchTer    => "Terminator in files does not match to selected one",
            AlphanumSep => "Alphanumeric value is selected for separator, \
                            please change it to other values",
            AlphanumTer => "Alphanumeric value is selected for terminator, \
                            please change it to other values",
            MultipleSep => "Separator cannot be more than one character, \
                            except for tab space `\\t`",
            MultipleTer => "Terminator cannot be more than one character, \
                            except for tab space `\\n`",
            EmptyChar   => "Some of the values were empty",
        };
        Error { message: message.to_string() }
    }
}
