#[macro_use]
extern crate kvs;

use kvs::{KvStore, KvsEngine, Result};
use std::env::current_dir;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "kvs-client",
    about = "The internal client implementation of KvStore, accessed via the command line"
)]
enum KvsClient {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let mut store = KvStore::open(current_dir()?)?;
    let config = KvsClient::from_args();
    let mut exit_code = 0;

    match config {
        KvsClient::Get { key } => match store.get(key) {
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
        KvsClient::Set { key, value } => match store.set(key, value) {
            Ok(()) => {}
            Err(error) => {
                eprintln!(kvs_error!(), error);
                exit_code = 1;
            }
        },
        KvsClient::Rm { key } => match store.remove(key) {
            Ok(()) => {}
            Err(error) => {
                println!(kvs_error!(), error);
                exit_code = 1;
            }
        },
    };
    exit(exit_code)
}
