use failure::Fail;
use std::convert::From;
use std::io;
use std::result;

/// # KvsError
/// This error is the user-facing error type for the KVS tool.
#[derive(Fail, Debug)]
#[fail(display = "{}", error_message)]
pub struct KvsError {
    /// The original error message as a string
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

impl KvsError {
    /// Builds a `KvsError` from some string-like value.
    ///
    /// ## Usage
    /// ```
    /// use kvs::KvsError;
    /// let error = KvsError::from_string("Key not found.");
    /// let other_error = KvsError::from_string(String::from("This also works"));
    /// # assert_eq!(error.error_message, String::from("Key not found."));
    /// ```
    pub fn from_string(error_message: impl Into<String>) -> Self {
        KvsError {
            error_message: error_message.into(),
        }
    }
}

/// # Result
/// This is simply an alias to prevent the overuse
/// of `Result<T, KvsError>`, since that will be
/// used throughout the codebase.
pub type Result<T> = result::Result<T, KvsError>;
