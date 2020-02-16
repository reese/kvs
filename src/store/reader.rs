use crate::store::{Entry, Result};
use std::collections::HashMap;
use std::fs::{read, File};

use crate::KvsError;
use std::borrow::BorrowMut;
use std::io;
use std::io::{BufRead, BufReader, Error, Read, Seek, SeekFrom};
use std::ops::Deref;
use std::path::PathBuf;

#[derive(Debug)]
pub struct BufReaderWithPosition<R: Read> {
    reader: BufReader<R>,
    position: u64,
}

impl<R: Read> BufReaderWithPosition<R> {
    pub fn new(file: R) -> Result<Self> {
        Ok(BufReaderWithPosition {
            reader: BufReader::new(file),
            position: 0,
        })
    }
}

impl<R: Read + Seek> Read for BufReaderWithPosition<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let buffer_len = self.reader.read(buf)?;
        self.position += buffer_len as u64;
        Ok(buffer_len)
    }
}

impl<R: Read + Seek> Seek for BufReaderWithPosition<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let position = self.reader.seek(pos)?;
        self.position = position;
        Ok(position)
    }
}
