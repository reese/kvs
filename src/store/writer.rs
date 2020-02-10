use crate::store::{get_directory_files_ascending, Entry};
use crate::{KvsError, ParsePath, Result};
use std::fs::{create_dir, read_dir, File};
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Debug)]
pub struct BufWriterWithPosition {
    directory: PathBuf,
    writer: Option<BufWriter<File>>,
    last_index: u64,
}

impl BufWriterWithPosition {
    pub fn new(directory: PathBuf) -> Result<Self> {
        let mut directory_files =
            get_directory_files_ascending(directory.clone())?;

        let final_index: Option<Result<u64>> = directory_files
            .last()
            .map(|last_path| last_path.parse_number_from_path());

        Ok(BufWriterWithPosition {
            directory,
            writer: None,
            last_index: final_index.unwrap_or(Ok(0))?,
        })
    }

    pub fn append_to_log(&mut self, entry: &Entry) -> Result<u64> {
        let file_index = self.last_index + 1;
        self.open_file(file_index)?;
        self.last_index = file_index;
        serde_json::to_writer(self.writer.as_ref().unwrap().get_ref(), entry)
            .map_err(KvsError::from)?;

        self.last_index += 1;
        Ok(file_index)
    }

    fn open_file(&mut self, file_index: u64) -> Result<()> {
        self.writer = Some(BufWriter::new(
            File::open(self.get_path_for_index(file_index))
                .or(File::create(self.get_path_for_index(file_index)))
                .map_err(KvsError::from)?,
        ));
        Ok(())
    }

    fn get_path_for_index(&self, file_index: u64) -> PathBuf {
        let mut path_buf = self.directory.clone();
        path_buf.push(format!("{}.log", file_index));
        path_buf
    }
}
