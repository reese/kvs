use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub struct BufReaderWithPosition {
    reader: HashMap<u64, BufReader<File>>,
    position: u64,
}

impl BufReaderWithPosition {
    pub fn new() -> Self {
        BufReaderWithPosition {
            reader: HashMap::new(),
            position: 0,
        }
    }
}
