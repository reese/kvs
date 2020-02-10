mod entry;
mod error;
mod path_buf;
mod reader;
mod writer;

pub use entry::Entry;
pub use error::{KvsError, Result};
pub use path_buf::ParsePath;
pub use reader::BufReaderWithPosition;
use std::fs::read_dir;
use std::path::PathBuf;
pub use writer::BufWriterWithPosition;

pub fn get_directory_files_ascending(
    directory: PathBuf,
) -> Result<Vec<PathBuf>> {
    let mut directory_files = read_dir(directory.clone())?
        .map(|file| file.unwrap().path())
        .collect::<Vec<_>>();
    directory_files.sort_unstable();
    Ok(directory_files)
}
