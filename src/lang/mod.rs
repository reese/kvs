//! # Lang
//! This module stores the raw strings used for
//! messages displayed by the CLI.
//!
//! ### Note on macro usage:
//! Note: The reason for having these ridiculous macros here
//! is to have the error messages be reusable and standardized.
//! However, macros like `println!` require string literals, so
//! this a terrible way of getting around that. In an ideal world,
//! there would be a macro to build these macros, but it's not particularly
//! important in the short term.

/// error message literal
#[macro_export]
macro_rules! successful_set {
    () => {
        "[KVS] Successfully stored key={} and value={}."
    };
}

/// error message literal
#[macro_export]
macro_rules! successful_rm {
    () => {
        "[KVS] Successfully removed key={}"
    };
}

/// error message literal
#[macro_export]
macro_rules! successful_get_with_result {
    () => {
        "[KVS] key={} value={}"
    };
}

/// error message literal
#[macro_export]
macro_rules! successful_get_without_result {
    () => {
        "[KVS] No value found for key={}"
    };
}

/// error message literal
#[macro_export]
macro_rules! kvs_error {
    () => {
        "[KVS] ERROR: {}"
    };
}
