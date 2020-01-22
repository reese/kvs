extern crate kvs;

use kvs::{default_path, KvStore, KvsError, Result};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Config {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let mut store = KvStore::open(default_path())?;

    match Config::from_args() {
        Config::Get { key } => {
            let string_option = store.get(key)?;
            println!(
                "{}",
                string_option.ok_or(KvsError {
                    error_message: String::from("Key not found")
                })?
            );
            Ok(())
        }
        Config::Set { key, value } => store.set(key, value),
        Config::Rm { key } => store.remove(key),
    }
}
