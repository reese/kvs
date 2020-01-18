use failure::Fail;
use std::result;

/// # KvsError
/// This error is the user-facing error type for the KVS tool.
#[derive(Fail, Debug)]
#[fail(display = "KVS encountered an error.")]
pub struct KvsError;

/// # Result
/// This is simply an alias to prevent the overuse
/// of `Result<T, KvsError>`, since that will be
/// used throughout the codebase.
pub type Result<T> = result::Result<T, KvsError>;
