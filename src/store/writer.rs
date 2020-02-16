use crate::store::{get_directory_files_descending, Entry, Position};
use crate::{KvsError, ParsePath, Result};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Error, Seek, Write};
use std::path::PathBuf;

#[derive(Debug)]
pub struct BufWriterWithPosition<W: Write + Seek> {
    directory: PathBuf,
    writer: BufWriter<W>,
    pub position: u64,
}

impl<W: Write + Seek> BufWriterWithPosition<W> {
    pub fn new(
        directory: PathBuf,
        next_command_position: u64,
    ) -> Result<BufWriterWithPosition<File>> {
        let directory_files =
            get_directory_files_descending(directory.clone())?;
        let mut new_log_path = directory.clone();
        new_log_path.push(format!("{}.log", next_command_position));

        Ok(BufWriterWithPosition {
            directory,
            writer: BufWriter::new(File::create(new_log_path)?),
            position: 0,
        })
    }

    fn get_path_for_index(&self, file_index: u64) -> PathBuf {
        let mut path_buf = self.directory.clone();
        path_buf.push(format!("{}.log", file_index));
        path_buf
    }
}

impl<W: Write + Seek> Write for BufWriterWithPosition<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let write_length = self.writer.write(buf)?;
        self.position += write_length as u64;
        Ok(write_length)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
