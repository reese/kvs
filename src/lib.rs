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
    BufReaderWithPosition, BufWriterWithPosition, Entry, Permissions,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{read_dir, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
pub use store::{KvsError, Result};

/// This is a placeholder for the current location of the
/// log file. Eventually, there should be a config or
/// environment variable to set this.
pub static TEST_FILE_PATH: &str = "/home/reese/src/kvs/fake_log_dir";

/// Getter for the default log file path.
pub fn default_path() -> PathBuf {
    PathBuf::from(TEST_FILE_PATH)
}

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
    pub fn open<'store>(path: impl Into<PathBuf> + Clone) -> Result<KvStore> {
        let path_buf: PathBuf = path.clone().into();
        let directory = read_dir(path_buf.clone())?;
        let mut log_files = directory
            .flat_map(|dir| dir.map(|entries| entries.path()))
            .collect::<Vec<_>>();
        log_files.sort_unstable();

        Ok(KvStore {
            dir: path.clone().into(),
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
    ///
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let new_last_index: u64 = self.get_last_index()?;
        self.writer.store_action_at_index(
            &self.dir,
            new_last_index,
            &Entry::set(key, value),
        )
    }

    /// Retrieves a value from the store.
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("name"), String::from("Caroline"));
    ///
    /// let name = store.get(String::from("name")).expect("Name was not found in store.");
    /// println!("Her name is {}", name); // => "Her name is Caroline"
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
                Err(error) => panic!("Error while reading from log."),
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
        // TODO: Append remove file to log
        Ok(())
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
}

pub mod lang;
mod store;
