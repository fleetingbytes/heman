//! Heman Errors.

use std::fmt;

/// An error that occurred during a query of the HTTP status code registry.
#[derive(Debug)]
pub enum Error {
    /// Raised when the status code has was not found in the HTTP registry.
    Unassigned,
    /// Raised when a substring was not found in any of the status code descriptions.
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
