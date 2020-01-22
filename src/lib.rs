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
use crate::store::{Entry, Permissions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::Path;
pub use store::{KvsError, Result};

/// This is a placeholder for the current location of the
/// log file. Eventually, there should be a config or
/// environment variable to set this.
pub static TEST_FILE_PATH: &str = "/home/reese/src/kvs/test.log";
/// Getter for the default log file path.
pub fn default_path() -> &'static Path {
    Path::new(TEST_FILE_PATH)
}

/// This struct serves as the main interface for storing and retrieving
/// data from the store. As of right now, it only stores things in-memory,
/// but this will be changed in coming updates.
#[derive(Debug, Deserialize, Serialize)]
pub struct KvStore<'store> {
    store: Vec<Entry>,
    /// The path of the log file on disk.
    #[serde(skip, default = "default_path")]
    pub path: &'store Path,
}

impl<'kv> KvStore<'kv> {
    /// Creates a new store with no keys or values.
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("key"), String::from("value"));
    /// ```
    pub fn new() -> Self {
        KvStore::new_with_path(Path::new(TEST_FILE_PATH))
    }

    /// Creates an empty store with an assigned file path.
    /// ```rust
    /// use kvs::KvStore;
    /// use std::path::Path;
    /// let store = KvStore::new_with_path(Path::new("/your/path/some.log"));
    /// assert_eq!(store.path, "/your/path/some.log")
    /// ```
    pub fn new_with_path(path: &'kv Path) -> Self {
        KvStore {
            store: vec![],
            path,
        }
    }

    /// Opens log file and replays entire log from the beginning.
    pub fn open(path: &Path) -> Result<KvStore> {
        let file = KvStore::get_file_permissions(
            Permissions::NotAllowed,
            Permissions::Allowed,
        )
        .open(TEST_FILE_PATH)?;
        let reader = BufReader::new(file);
        let store: Result<Vec<Entry>> =
            serde_json::from_reader(reader).map_err(KvsError::from);
        match store {
            Ok(entries) => Ok(KvStore {
                store: entries,
                path,
            }),
            Err(_error) => Ok(KvStore::new_with_path(path)),
        }
    }

    /// Sets a new value for the given key in the store.
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("module_name"), String::from("kvs"));
    /// ```
    ///
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.store.push(Entry::Set(key, value));
        self.write_store_to_file()
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
        self.store.iter().for_each(|entry| match entry {
            Entry::Set(key, value) => {
                final_state.insert(key.to_string(), value.to_string());
            }
            Entry::Rm(key) => {
                final_state.remove(key);
            }
            _ => {
                unreachable!();
            }
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
        self.store.push(Entry::Rm(key));
        self.write_store_to_file()
    }

    fn open_file(
        &self,
        write_permission: Permissions,
        read_permission: Permissions,
    ) -> Result<File> {
        KvStore::get_file_permissions(write_permission, read_permission)
            .open(self.path.clone())
            .map_err(KvsError::from)
    }

    fn get_file_permissions(
        write_permission: Permissions,
        read_permission: Permissions,
    ) -> OpenOptions {
        OpenOptions::new()
            .create(true)
            .append(!write_permission.is_allowed())
            .write(write_permission.is_allowed())
            .read(read_permission.is_allowed())
            .to_owned()
    }

    fn write_store_to_file(&self) -> Result<()> {
        let file =
            self.open_file(Permissions::Allowed, Permissions::Allowed)?;
        serde_json::to_writer(file, &self.store).map_err(KvsError::from)
    }
}

mod store;
