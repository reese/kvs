#[macro_use]
extern crate kvs;

use kvs::{default_path, KvStore, Result};
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Config {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let mut store = KvStore::open(default_path())?;
    let config = Config::from_args();
    let mut exit_code = 0;

    match config {
        Config::Get { key } => match store.get(key.clone()) {
            Ok(optional_string) => {
                if let Some(found_string) = optional_string {
                    println!(successful_get_with_result!(), key, found_string);
                } else {
                    eprintln!(successful_get_without_result!(), key);
                    exit_code = 1;
                }
            }
            Err(error) => {
                eprintln!(kvs_error!(), error);
                exit_code = 1;
            }
        },
        Config::Set { key, value } => {
            match store.set(key.clone(), value.clone()) {
                Ok(()) => {
                    println!(successful_set!(), key, value);
                }
                Err(error) => {
                    eprintln!(kvs_error!(), error);
                    exit_code = 1;
                }
            }
        }
        Config::Rm { key } => match store.remove(key.clone()) {
            Ok(()) => {
                println!(successful_rm!(), key);
            }
            Err(error) => {
                eprintln!(kvs_error!(), error);
                exit_code = 1;
            }
        },
    };
    exit(exit_code)
}
