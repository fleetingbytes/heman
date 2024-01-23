//use clap::error::Error as ClapError;
//use std::convert::From;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Unassigned,
    NotFound,
    //Clap(ClapError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Unassigned => write!(f, "not assigned"),
            Error::NotFound => write!(f, "not found"),
            //Error::Clap(clap_error) => write!(f, "clap error: {}", clap_error),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Unassigned => None,
            Error::NotFound => None,
            //Error::Clap(ref clap_error) => Some(clap_error),
        }
    }
}

//impl From<ClapError> for Error {
//fn from(value: ClapError) -> Self {
//Error::Clap(value)
//}
//}
