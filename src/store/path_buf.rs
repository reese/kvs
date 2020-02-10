use crate::store::Result;
use crate::KvsError;
use std::path::PathBuf;

/// Extracts the number from a given file stem
pub trait ParsePath {
    /// Extracts the number from a given file stem
    fn parse_number_from_path(&self) -> Result<u64>;
}

impl ParsePath for PathBuf {
    /// Extracts the number from a given file stem.
    /// For example, the file "123.log" would return `123`.
    ///
    /// ```rust
    /// use kvs::ParsePath;
    /// use std::path::PathBuf;;
    ///
    /// let path_buf = PathBuf::from("112902.log");
    /// assert!(path_buf.parse_number_from_path() == 112902);
    /// ```
    fn parse_number_from_path(&self) -> Result<u64> {
        self.file_stem()
            .expect("File stem could not be read.")
            .to_str()
            .expect("File stem could not be converted from OsString to &str.")
            .to_string()
            .parse::<u64>()
            .map_err(KvsError::from)
    }
}
