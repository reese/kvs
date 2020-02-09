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
use crate::store::{BufReaderWithPosition, BufWriterWithPosition, Entry};
use std::collections::HashMap;
use std::fs::{create_dir, read_dir, File};
use std::path::PathBuf;
pub use store::{KvsError, Result};

/// This struct serves as the main interface for storing and retrieving
/// data from the store. As of right now, it only stores things in-memory,
/// but this will be changed in coming updates.
#[derive(Debug)]
pub struct KvStore {
    dir: PathBuf,
    store: HashMap<String, String>,
    /// The paths of all files in the log.
    pub paths: Vec<PathBuf>,
    reader: BufReaderWithPosition,
    writer: BufWriterWithPosition,
}

impl KvStore {
    /// Opens log file and replays entire log from the beginning.
    pub fn open(path: impl Into<PathBuf> + Clone) -> Result<KvStore> {
        let mut path_buf: PathBuf = path.clone().into();
        path_buf.push(".kvs");
        if !path_buf.exists() {
            create_dir(path_buf.clone()).map_err(KvsError::from)?;
        }
        let directory = read_dir(path_buf.clone())?;
        let mut log_files = directory
            .flat_map(|dir| dir.map(|entries| entries.path()))
            .collect::<Vec<_>>();
        log_files.sort_unstable();

        Ok(KvStore {
            dir: path_buf.clone(),
            paths: log_files,
            reader: BufReaderWithPosition::new(),
            store: HashMap::new(),
            writer: BufWriterWithPosition::new(),
        })
    }

    /// Sets a new value for the given key in the store.
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("module_name"), String::from("kvs"));
    /// ```
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let new_last_index: u64 = self.get_last_index()?;
        let result = self.writer.store_action_at_index(
            &self.dir,
            new_last_index,
            &Entry::set(key, value),
        );
        if result.is_ok() {
            self.add_index_to_paths(new_last_index);
        }
        result
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
    pub fn get(&self, key: String) -> Result<Option<String>> {
        let mut final_state: HashMap<String, String> = HashMap::new();
        self.paths
            .iter()
            .map(|file| {
                serde_json::from_reader(File::open(file)?)
                    .map_err(KvsError::from)
            })
            .for_each(|entry| match entry {
                Ok(Entry::Set(key, value)) => {
                    final_state.insert(key, value);
                }
                Ok(Entry::Rm(key)) => {
                    final_state.remove(key.as_str());
                }
                Err(_error) => panic!("Error while reading from log."),
            });

        Ok(final_state.get(&key).cloned())
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

        let new_last_index: u64 = self.get_last_index()?;
        let result = self.writer.store_action_at_index(
            &self.dir,
            new_last_index,
            &Entry::rm(key),
        );

        if result.is_ok() {
            self.add_index_to_paths(new_last_index);
        }

        result
    }

    fn get_last_index(&self) -> Result<u64> {
        let last_index = self.paths.last().map(|last_path| {
            last_path
                .file_stem()
                .expect("File stem could not be read.")
                .to_str()
                .expect(
                    "File stem could not be converted from OsString to &str.",
                )
                .to_string()
                .parse::<u64>()
                .map_err(KvsError::from)
        });

        if let Some(index) = last_index {
            match index {
                Ok(num) => Ok(num + 1),
                Err(err) => Err(err),
            }
        } else {
            Ok(0)
        }
    }
    fn add_index_to_paths(&mut self, new_last_index: u64) {
        self.paths
            .push(self.writer.get_path_for_index(&self.dir, new_last_index));
    }
}

pub mod lang;
mod store;
