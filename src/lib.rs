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
//!

#[macro_use]
extern crate failure;
extern crate serde;
use crate::store::{
    get_directory_files_ascending, BufReaderWithPosition,
    BufWriterWithPosition, Entry,
};
use std::collections::HashMap;
use std::fs::create_dir;
use std::path::PathBuf;
pub use store::{KvsError, ParsePath, Result};

/// This struct serves as the main interface for storing and retrieving
/// data from the store. As of right now, it only stores things in-memory,
/// but this will be changed in coming updates.
#[derive(Debug)]
pub struct KvStore {
    store: HashMap<String, u64>,
    reader: BufReaderWithPosition,
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

        Ok(KvStore {
            reader: BufReaderWithPosition::new(path_buf.clone()),
            store: HashMap::new(),
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
        let result = self.writer.append_to_log(&Entry::set(key.clone(), value));
        match result.clone() {
            Ok(new_last_index) => {
                self.update_index(new_last_index, key);
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
        if let Some(retrieved_value) = index {
            match self.reader.read_index(*retrieved_value) {
                Ok(Entry::Set(value, ..)) => Ok(Some(value)),
                Ok(Entry::Rm(..)) => Ok(None),
                Err(error) => Err(KvsError::from(error)),
            }
        } else {
            let entry =
                get_directory_files_ascending(self.reader.get_directory())?
                    .iter()
                    .find(|path| {
                        self.reader
                            .read_index(
                                path.parse_number_from_path().expect(
                                    "Could not parse index from file name",
                                ),
                            )
                            .unwrap()
                            .get_key()
                            .eq(&key)
                    })
                    .map(|path| {
                        self.reader.read_index(path.parse_number_from_path()?)
                    })
                    .or_else(|| None);
            match entry {
                Some(Ok(Entry::Rm(..))) => Ok(None),
                Some(Ok(Entry::Set(value, ..))) => Ok(Some(value)),
                Some(Err(error)) => Err(error),
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
        let result = self.writer.append_to_log(&Entry::rm(key.clone()));

        match result.clone() {
            Ok(new_last_index) => {
                self.update_index(new_last_index, key);
                Ok(())
            }
            Err(error) => Err(error),
        }
    }

    fn update_index(&mut self, new_last_index: u64, key: String) {
        self.store.insert(key, new_last_index);
    }
}

pub mod lang;
mod store;
