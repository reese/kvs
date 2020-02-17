#[macro_use]
extern crate kvs;

use kvs::{KvStore, Result};
use std::env::current_dir;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Config {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let mut store = KvStore::open(current_dir()?)?;
    let config = Config::from_args();
    let mut exit_code = 0;

    match config {
        Config::Get { key } => match store.get(key) {
            Ok(optional_string) => {
                if let Some(found_string) = optional_string {
                    println!(successful_get_with_result!(), found_string);
                } else {
                    println!(successful_get_without_result!());
                }
            }
            Err(error) => {
                eprintln!(kvs_error!(), error);
                exit_code = 1;
            }
        },
        Config::Set { key, value } => match store.set(key, value) {
            Ok(()) => {}
            Err(error) => {
                eprintln!(kvs_error!(), error);
                exit_code = 1;
            }
        },
        Config::Rm { key } => match store.remove(key) {
            Ok(()) => {}
            Err(error) => {
                println!(kvs_error!(), error);
                exit_code = 1;
            }
        },
    };
    exit(exit_code)
}
