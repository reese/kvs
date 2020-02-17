use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    Set(String, String),
    Rm(String),
}

impl Entry {
    pub fn rm(key: String) -> Self {
        Entry::Rm(key)
    }

    pub fn set(key: String, value: String) -> Self {
        Entry::Set(key, value)
    }

    pub fn get_key(&self) -> &String {
        match self {
            Entry::Set(key, ..) => key,
            Entry::Rm(key) => key,
        }
    }
}
