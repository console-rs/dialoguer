use std::{io::Error as IoError, result::Result as StdResult};

use thiserror::Error;

/// Possible errors returned by prompts.
#[derive(Error, Debug)]
pub enum Error {
    /// Error while executing IO operations.
    #[error("IO error: {0}")]
    IO(#[from] IoError),
}

/// Result type where errors are of type [Error](enum@Error).
pub type Result<T = ()> = StdResult<T, Error>;

impl From<Error> for IoError {
    fn from(value: Error) -> Self {
        match value {
            Error::IO(err) => err,
            // If other error types are added in the future:
            // err => IoError::new(std::io::ErrorKind::Other, err),
        }
    }
}
