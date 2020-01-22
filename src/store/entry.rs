use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    Set(String, String),
    Get(String),
    Rm(String),
}
