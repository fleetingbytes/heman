use std::convert::From;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    Parse(ParseIntError),
    NotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Parse(parse_int_error) => write!(f, "{}", parse_int_error),
            Error::NotFound => write!(f, "not found"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Parse(ref parse_int_error) => Some(parse_int_error),
            Error::NotFound => None,
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::Parse(value)
    }
}
