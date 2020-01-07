use std::fmt;
use std::io::Error as IOError;
use std::num;
use std::process;
use std::result;

pub mod input_list;

#[derive(Debug)]
pub enum ErrorRepr {
    IOError(IOError),
    ParseError(String),
    Other(String),
}

#[derive(Debug)]
pub struct Error {
    repr: ErrorRepr,
}

impl From<IOError> for Error {
    fn from(io_error: IOError) -> Self {
        Error {
            repr: ErrorRepr::IOError(io_error),
        }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(_err: num::ParseIntError) -> Self {
        Error {
            repr: ErrorRepr::ParseError(String::from("numbers")),
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(error_message: &'a str) -> Self {
        Error {
            repr: ErrorRepr::Other(String::from(error_message)),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        match self.repr {
            ErrorRepr::IOError(ref io_error) => io_error.fmt(f),
            ErrorRepr::ParseError(ref dest) => {
                write!(f, "Failed to parse input list into a list of {}", dest)
            }
            ErrorRepr::Other(ref message) => write!(f, "{}", message),
        }
    }
}

pub type Result = result::Result<(), Error>;

pub trait DayRunner {
    fn run(&self, args: &Vec<String>) -> Result;
}

pub const NO_DAY_PROVIDED_EXIT_CODE: i32 = 1;
pub const BAD_DAY_PROVIDED_EXIT_CODE: i32 = 2;

/// Exit out with a specific error message and exit code
pub fn fail(error_message: &str, exit_code: i32) -> ! {
    // Print out the error message to stderr
    eprintln!("{}", error_message);

    // Exit with the appropriate exit code
    process::exit(exit_code)
}
