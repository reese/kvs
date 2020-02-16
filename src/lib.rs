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
    get_directory_files_ascending, BufReaderWithPosition,
    BufWriterWithPosition, Entry,
};
use std::collections::{BTreeMap, HashMap};
use std::fs::{create_dir, File, ReadDir};
use std::io::Read;
use std::ops::Deref;
use std::path::PathBuf;
pub use store::{KvsError, ParsePath, Result};

/// This struct serves as the main interface for storing and retrieving
/// data from the store. It uses a log-based file structure to store
/// values on disk and a log-pointer cache to store the latest references
/// to given keys.
#[derive(Debug)]
pub struct KvStore {
    directory: PathBuf,
    store: BTreeMap<String, u64>,
    reader_map: HashMap<u64, BufReaderWithPosition<File>>,
    writer: BufWriterWithPosition,
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
                reader_map.insert(
                    path.parse_number_from_path().unwrap(),
                    BufReaderWithPosition::new(File::open(path).unwrap())
                        .unwrap(),
                );
            });

        Ok(KvStore {
            directory: path_buf.clone(),
            reader_map,
            store,
            writer: BufWriterWithPosition::new(path_buf)?,
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
        let result = self.writer.append_to_log(&new_entry);
        match result.clone() {
            Ok(new_last_index) => {
                self.store.insert(key, new_last_index);
                Ok(())
            }
            Err(error) => Err(error),
        }
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
                Ok(Entry::Set(value, ..)) => Ok(Some(value.parse().unwrap())),
                Err(error) => Err(error),
            }
        } else {
            let entry = get_directory_files_ascending(self.directory.clone())?
                .iter()
                .find(|path| {
                    self.read_index(
                        path.parse_number_from_path()
                            .expect("Could not parse index from file name"),
                    )
                    .unwrap()
                    .get_key()
                    .eq(&key)
                })
                .map(|path| {
                    let index_num = path
                        .parse_number_from_path()
                        .expect("Could not parse number from file stem");
                    self.read_index(index_num.clone())
                        .expect("Could not read item at index")
                })
                .or_else(|| None);
            match entry {
                Some(Entry::Rm(..)) => Ok(None),
                Some(Entry::Set(value, ..)) => Ok(Some(value)),
                _ => Ok(None),
            }
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
            return Err(KvsError::from_string("Key not found"));
        }
        let entry = Entry::rm(key.clone());
        let result = self.writer.append_to_log(&entry);

        match result.clone() {
            Ok(new_last_index) => {
                self.store.insert(key, new_last_index);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn read_index(&mut self, index: u64) -> Result<Entry> {
        let reader_option = self.reader_map.get_mut(&index);
        if reader_option.is_some() {
            serde_json::from_reader(reader_option.unwrap())
                .map_err(KvsError::from)
        } else {
            if !self.get_path_for_index(index).exists() {
                Err(KvsError::from_string("No file exists at the given index."))
            } else {
                self.reader_map.insert(
                    index,
                    BufReaderWithPosition::new(File::open(
                        self.get_path_for_index(index),
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
}

pub mod lang;
mod store;
