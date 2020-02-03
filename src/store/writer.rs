use crate::store::Entry;
use crate::{KvsError, Result};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

#[derive(Debug)]
pub struct BufWriterWithPosition {
    writer: Option<BufWriter<File>>,
    position: u64,
}

impl BufWriterWithPosition {
    pub fn new() -> Self {
        BufWriterWithPosition {
            writer: None,
            position: 0,
        }
    }

    pub fn store_action_at_index(
        &mut self,
        dir: &PathBuf,
        file_index: u64,
        entry: &Entry,
    ) -> Result<()> {
        self.open_file(dir, file_index)?;
        self.position = file_index;
        serde_json::to_writer(self.writer.as_ref().unwrap().get_ref(), entry)
            .map_err(KvsError::from)
    }

    pub fn open_file(&mut self, dir: &PathBuf, file_index: u64) -> Result<()> {
        self.writer = Some(BufWriter::new(
            File::open(self.get_path_for_index(dir, file_index))
                .or(File::create(self.get_path_for_index(dir, file_index)))
                .map_err(KvsError::from)?,
        ));
        Ok(())
    }

    fn get_path_for_index(&self, dir: &PathBuf, file_index: u64) -> PathBuf {
        let mut path_buf = dir.clone();
        path_buf.push(format!("{}.log", file_index));
        path_buf
    }
}
