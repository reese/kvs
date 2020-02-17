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
//! TODO: Benchmarks should also be added to Github Action.
//!

#[macro_use]
extern crate failure;
extern crate serde;
use crate::store::{
    BufReaderWithPosition, BufWriterWithPosition, Entry, Position,
};

use serde_json::Deserializer;
use std::collections::{BTreeMap, HashMap};
use std::fs::{create_dir, File};
use std::io::{Read, Seek, SeekFrom, Write};

use std::path::PathBuf;

use std::fs;
pub use store::{KvsError, ParsePath, Result};

const COMPACTION_MINIMUM: u64 = 500;

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
    compaction_counter: u64,
}

impl KvStore {
    /// Initializes `KvStore` readers and writers.
    pub fn open(path: impl Into<PathBuf> + Clone) -> Result<KvStore> {
        let mut path_buf: PathBuf = path.into();
        path_buf.push(".kvs");
        if !path_buf.exists() {
            create_dir(path_buf.clone()).map_err(KvsError::from)?;
        }

        let mut store = BTreeMap::new();
        let mut reader_map = HashMap::new();

        get_descending_files_in_directory(path_buf.clone())
            .iter()
            .for_each(|path| {
                let mut buffer = BufReaderWithPosition::new(
                    File::open(path.clone()).unwrap(),
                )
                .unwrap();
                load_entry(path.clone(), &mut store, &mut buffer).unwrap();
                reader_map
                    .insert(path.parse_number_from_path().unwrap(), buffer);
            });

        let next_command_position =
            reader_map.keys().max().map(|num| num + 1).unwrap_or(0);

        Ok(KvStore {
            directory: path_buf.clone(),
            reader_map,
            store,
            writer: BufWriterWithPosition::<File>::create(
                path_buf,
                next_command_position,
            )?,
            next_command_position,
            compaction_counter: 0,
        })
    }

    /// Sets a new value for the given key in the store.
    /// ```rust
    /// use kvs::KvStore;
    /// use std::env::temp_dir;
    /// let mut store = KvStore::open(temp_dir()).unwrap();
    /// store.set(String::from("module_name"), String::from("kvs"));
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let new_entry = Entry::set(key, value);
        self.append_entry(new_entry)
    }

    /// Retrieves a value from the store.
    /// ```rust
    /// use kvs::KvStore;
    /// use std::env::temp_dir;
    /// let mut store = KvStore::open(temp_dir()).unwrap();
    /// store.set(String::from("name"), String::from("Caroline"));
    ///
    /// let name = store.get(String::from("name")).expect("Name was not found in store.").unwrap();
    /// assert_eq!(name, String::from("Caroline"));
    /// ```
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        let index = self.store.get(key.as_str());
        if let Some(position) = index {
            // This clone is to get around a mutable borrow reservation conflict.
            // For more info, see the tracking issue: https://github.com/rust-lang/rust/issues/59159
            let cloned_position = *position;
            match self.read_index(cloned_position) {
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
    /// use std::env::temp_dir;
    /// let mut store = KvStore::open(temp_dir()).unwrap();
    /// store.set(String::from("album_name"), String::from("Blood Type"));
    /// store.remove(String::from("album_name"));
    /// assert!(store.get(String::from("album_name")).unwrap().is_none());
    /// ```
    pub fn remove(&mut self, key: String) -> Result<()> {
        let is_existing_value = self.get(key.clone())?.is_some();
        if !is_existing_value {
            Err(KvsError::from_string("Key not found"))
        } else {
            let entry = Entry::rm(key);
            self.append_entry(entry)
        }
    }

    fn read_index(&mut self, index: Position) -> Result<Entry> {
        let reader_option = self.reader_map.get_mut(&index.file_index);
        if let Some(buffer) = reader_option {
            buffer.seek(SeekFrom::Start(index.start_position))?;
            buffer.take(index.length);
            serde_json::from_reader(buffer).map_err(KvsError::from)
        } else if !self.get_path_for_index(index.file_index).exists() {
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

    fn compact_log(&mut self) -> Result<()> {
        let mut directory_files: Vec<PathBuf> =
            get_descending_files_in_directory(self.directory.clone());
        directory_files.reverse();

        let mut final_keys = HashMap::new();
        directory_files.iter().for_each(|path| {
            let key: Entry =
                serde_json::from_reader(File::open(path).unwrap()).unwrap();
            if final_keys.get(key.get_key()).is_some() {
                fs::remove_file(path).expect("Could not delete file.");
                self.reader_map
                    .remove(&path.parse_number_from_path().unwrap());
            } else {
                final_keys.insert(key.get_key().clone(), true);
            }
        });
        self.compaction_counter = 0;

        Ok(())
    }

    fn get_path_for_index(&self, index: u64) -> PathBuf {
        let mut directory = self.directory.clone();
        directory.push(format!("{}.log", index));
        directory
    }

    fn append_entry(&mut self, new_entry: Entry) -> Result<()> {
        self.writer = BufWriterWithPosition::<File>::create(
            self.directory.clone(),
            self.next_command_position,
        )?;
        let start_position = self.writer.position;
        serde_json::to_writer(&mut self.writer, &new_entry)?;
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
        self.compaction_counter += 1;

        if self.compaction_counter > COMPACTION_MINIMUM {
            self.compact_log()?;
        }

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

fn get_descending_files_in_directory(directory: PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = directory
        .read_dir()
        .expect("Could not read files in directory.")
        .map(|dir_entry| dir_entry.unwrap().path())
        .collect();
    files.sort_unstable_by_key(|path| path.parse_number_from_path().unwrap());
    files
}

pub mod lang;
mod store;
