use failure::Fail;
use std::convert::From;
use std::error::Error;
use std::io;
use std::result;

/// # KvsError
/// This error is the user-facing error type for the KVS tool.
#[derive(Fail, Debug)]
#[fail(display = "KVS encountered the following error: {}", error_message)]
pub struct KvsError {
    /// The error message dummy
    pub error_message: String,
}

impl From<io::Error> for KvsError {
    fn from(error: io::Error) -> Self {
        KvsError {
            error_message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(error: serde_json::Error) -> Self {
        KvsError {
            error_message: error.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for KvsError {
    fn from(error: std::num::ParseIntError) -> Self {
        KvsError {
            error_message: error.to_string(),
        }
    }
}

/// # Result
/// This is simply an alias to prevent the overuse
/// of `Result<T, KvsError>`, since that will be
/// used throughout the codebase.
pub type Result<T> = result::Result<T, KvsError>;
