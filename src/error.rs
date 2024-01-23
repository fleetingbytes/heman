use std::fmt;

#[derive(Debug)]
pub enum Error {
    Unassigned,
    NotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::Unassigned => write!(f, "not assigned"),
            Error::NotFound => write!(f, "not found"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::Unassigned => None,
            Error::NotFound => None,
        }
    }
}
