use crate::store::{Entry, Result};
use std::collections::HashMap;
use std::fs::File;

use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug)]
pub struct BufReaderWithPosition {
    directory: PathBuf,
    reader: HashMap<u64, BufReader<File>>,
    position: u64,
}

impl BufReaderWithPosition {
    pub fn new(directory: PathBuf) -> Self {
        BufReaderWithPosition {
            directory,
            reader: HashMap::new(),
            position: 0,
        }
    }

    pub fn read_index(&mut self, index: u64) -> Result<Entry> {
        if let Some(reader) = self.reader.get(&index) {
            Ok(serde_json::from_reader(reader.get_ref())?)
        } else {
            self.reader.insert(
                index,
                BufReader::new(File::open(self.get_path_for_index(index))?),
            );
            self.read_index(index)
        }
    }

    pub fn get_directory(&self) -> PathBuf {
        self.directory.clone()
    }

    fn get_path_for_index(&self, index: u64) -> PathBuf {
        let mut dir = self.directory.clone();
        dir.push(format!("{}.log", index));
        dir
    }
}
