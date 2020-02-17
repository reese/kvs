#[macro_use]
extern crate kvs;
#[macro_use]
extern crate log;
extern crate stderrlog;

use kvs::{KvStore, KvsEngine, Options, Result};
use std::env::current_dir;
use std::net::SocketAddr;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "kvs-client",
    about = "The internal client implementation of KvStore, accessed via the command line"
)]
struct KvsClient {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(flatten)]
    options: Options,
}

#[derive(StructOpt, Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    stderrlog::new().init().unwrap();
    let config = KvsClient::from_args();
    info!("KvsClient version: {}", env!("CARGO_PKG_VERSION"));
    info!("Listening on port: {}", config.options.socket);
    info!("Running on engine: {}", config.options.engine);

    let mut store = KvStore::open(current_dir()?)?;
    let mut exit_code = 0;

    match config.command {
        Command::Get { key, .. } => match store.get(key) {
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
        Command::Set { key, value, .. } => match store.set(key, value) {
            Ok(()) => {}
            Err(error) => {
                eprintln!(kvs_error!(), error);
                exit_code = 1;
            }
        },
        Command::Rm { key, .. } => match store.remove(key) {
            Ok(()) => {}
            Err(error) => {
                println!(kvs_error!(), error);
                exit_code = 1;
            }
        },
    };
    exit(exit_code)
}
