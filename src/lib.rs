#![deny(missing_docs)]
//! # KVS
//! `KVS` is a key-value store used in the `kvs` command-line utility.
//!
use std::collections::HashMap;

/// This struct serves as the main interface for storing and retrieving
/// data from the store. As of right now, it only stores things in-memory,
/// but this will be changed in coming updates.
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new store with no keys or values.
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("key"), String::from("value"));
    /// ```
    pub fn new() -> Self {
        KvStore::default()
    }

    /// Sets a new value for the given key in the store.
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("module_name"), String::from("kvs"));
    /// ```
    ///
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
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
    pub fn get(&self, key: String) -> Option<String> {
        self.store.get::<String>(&key).cloned()
    }

    /// Removes the given key from the store.
    /// ```rust
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// store.set(String::from("album_name"), String::from("Blood Type"));
    /// store.remove(String::from("album_name"));
    /// ```
    pub fn remove(&mut self, key: String) {
        self.store.remove::<String>(&key);
    }
}
