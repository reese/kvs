#![deny(missing_docs)]
//! # KVS
//! `KVS` is a key-value store used in the `kvs` command-line utility.
//!
//! ## Serialization Format
//! This crate uses the `serde_json` crate for (de-)serialization.
//! For development purposes, it's helpful to have this in a standard, readable format.
//! Eventually, however, it will likely be in the best interest of performance
//! to change this for a different format.
//!
//! TODO: Benchmark different serialization formats.
//! TODO: Use benchmark tests to compare similar tools.
//!

#[macro_use]
extern crate failure;
extern crate serde;
use crate::store::{
    get_directory_files_descending, BufReaderWithPosition,
    BufWriterWithPosition, Entry, Position,
};
use serde_json::map::IntoIter;
use serde_json::Deserializer;
use std::collections::{BTreeMap, HashMap};
use std::fs::{create_dir, File, ReadDir};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ops::Deref;
use std::path::PathBuf;
use std::process::Command;
pub use store::{KvsError, ParsePath, Result};

/// This struct serves as the main interface for storing and retrieving
/// data from the store. It uses a log-based file structure to store
/// values on disk and a log-pointer cache to store the latest references
/// to given keys.
#[derive(Debug)]
pub struct KvStore {
    directory: PathBuf,
    store: BTreeMap<String, Position>,
    reader_map: HashMap<u64, BufReaderWithPosition<File>>,
    writer: BufWriterWithPosition<File>,
    next_command_position: u64,
}

impl KvStore {
    /// Initializes `KvStore` readers and writers.
    pub fn open(path: impl Into<PathBuf> + Clone) -> Result<KvStore> {
        let mut path_buf: PathBuf = path.clone().into();
        path_buf.push(".kvs");
        if !path_buf.exists() {
            create_dir(path_buf.clone()).map_err(KvsError::from)?;
        }

        let mut store = BTreeMap::new();
        let mut reader_map = HashMap::new();

        path_buf
            .read_dir()?
            .map(|dir_entry| dir_entry.unwrap().path())
            .for_each(|path| {
                let mut buffer = BufReaderWithPosition::new(
                    File::open(path.clone()).unwrap(),
                )
                .unwrap();
                load_entry(path.clone(), &mut store, &mut buffer);
                reader_map
                    .insert(path.parse_number_from_path().unwrap(), buffer);
            });

        let next_command_position =
            reader_map.keys().max().map(|pos| pos + 1).unwrap_or(0);

        Ok(KvStore {
            directory: path_buf.clone(),
            reader_map,
            store,
            writer: BufWriterWithPosition::<File>::new(
                path_buf,
                next_command_position,
            )?,
            next_command_position,
        })
    }

    /// Sets a new value for the given key in the store.
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("module_name"), String::from("kvs"));
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let new_entry = Entry::set(key.clone(), value.clone());
        self.append_entry(new_entry)
    }

    /// Retrieves a value from the store.
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("name"), String::from("Caroline"));
    ///
    /// let name = store.get(String::from("name")).expect("Name was not found in store.");
    /// assert!(name == String::from("Caroline"));
    /// ```
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let index = self.store.get(key.as_str());
        if let Some(value) = index {
            match self.read_index(*value) {
                Ok(Entry::Rm(..)) => Ok(None),
                Ok(Entry::Set(.., value)) => Ok(Some(value.parse().unwrap())),
                Err(error) => Err(error),
            }
        } else {
            Ok(None)
        }
    }

    /// Removes the given key from the store.
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("album_name"), String::from("Blood Type"));
    /// store.remove(String::from("album_name"));
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        let is_existing_value = self.get(key.clone())?.is_some();
        if !is_existing_value {
            Err(KvsError::from_string("Key not found"))
        } else {
            let entry = Entry::rm(key.clone());
            self.append_entry(entry)
        }
    }

    fn read_index(&mut self, index: Position) -> Result<Entry> {
        let reader_option = self.reader_map.get_mut(&index.file_index);
        if reader_option.is_some() {
            let buffer = reader_option.unwrap();
            buffer.seek(SeekFrom::Start(index.start_position))?;
            buffer.take(index.length);
            serde_json::from_reader(buffer).map_err(KvsError::from)
        } else {
            if !self.get_path_for_index(index.file_index).exists() {
                Err(KvsError::from_string("No file exists at the given index."))
            } else {
                self.reader_map.insert(
                    index.file_index,
                    BufReaderWithPosition::new(File::open(
                        self.get_path_for_index(index.file_index),
                    )?)?,
                );
                self.read_index(index)
            }
        }
    }

    fn get_path_for_index(&self, index: u64) -> PathBuf {
        let mut directory = self.directory.clone();
        directory.push(format!("{}.log", index));
        directory
    }

    fn append_entry(&mut self, new_entry: Entry) -> Result<()> {
        self.writer = BufWriterWithPosition::<File>::new(
            self.directory.clone(),
            self.next_command_position,
        )?;
        let start_position = self.writer.position;
        serde_json::to_writer(&mut self.writer, &new_entry);
        self.writer.flush()?;
        match new_entry {
            Entry::Set(key, ..) => {
                self.store.insert(
                    key,
                    (
                        self.next_command_position,
                        start_position,
                        self.writer.position,
                    )
                        .into(),
                );
            }
            Entry::Rm(key) => {
                self.store.remove(&key);
            }
        }
        self.next_command_position += 1;
        Ok(())
    }
}

fn load_entry(
    entry_path: PathBuf,
    store: &mut BTreeMap<String, Position>,
    reader: &mut BufReaderWithPosition<File>,
) -> Result<()> {
    let mut start_position = reader.seek(SeekFrom::Start(0))?;
    let mut stream = Deserializer::from_reader(reader).into_iter::<Entry>();
    while let Some(entry) = stream.next() {
        let end_position = stream.byte_offset();
        match entry.expect("Entry could not be deserialized.") {
            Entry::Set(key, ..) => {
                store.insert(
                    key,
                    (
                        entry_path.parse_number_from_path()?,
                        start_position,
                        end_position as u64,
                    )
                        .into(),
                );
            }
            Entry::Rm(key) => {
                store.remove(&key);
            }
        }
        start_position = end_position as u64;
    }
    Ok(())
}

pub mod lang;
mod store;
