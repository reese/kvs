use crate::store::{get_directory_files_ascending, Entry};
use crate::{KvsError, ParsePath, Result};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

#[derive(Debug)]
pub struct BufWriterWithPosition {
    directory: PathBuf,
    writer: Option<BufWriter<File>>,
    next_index: u64,
}

impl BufWriterWithPosition {
    pub fn new(directory: PathBuf) -> Result<Self> {
        let directory_files = get_directory_files_ascending(directory.clone())?;

        let final_index: Option<Result<u64>> = directory_files
            .last()
            .map(|last_path| last_path.parse_number_from_path());

        Ok(BufWriterWithPosition {
            directory,
            writer: None,
            next_index: final_index.map(|i| i.unwrap() + 1).unwrap_or(0),
        })
    }

    pub fn append_to_log(&mut self, entry: &Entry) -> Result<u64> {
        let file_index = self.next_index;
        self.open_file(file_index)?;
        serde_json::to_writer(self.writer.as_ref().unwrap().get_ref(), entry)
            .map_err(KvsError::from)?;

        self.next_index += 1;
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
