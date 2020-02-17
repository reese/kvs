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
    name = "kvs-server",
    about = "The redis server implementation for accessing the store over a network."
)]
struct KvsServer {
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
    stderrlog::new().module(module_path!()).init().unwrap();
    let config = KvsServer::from_args();

    warn!("KvsServer version: {}", env!("CARGO_PKG_VERSION"));
    warn!("Listening on port: {:?}", config.options.socket);
    warn!("Running on engine: {}", config.options.engine);

    let mut store = KvStore::open(current_dir()?)?;
    let mut exit_code = 0;

    match config.command {
        _ => warn!("some shit going on"),
    }

    exit(exit_code)
}
