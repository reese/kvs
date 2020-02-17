#[macro_use]
extern crate kvs;

use kvs::{KvStore, KvsEngine, Result};
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
    #[structopt(long = "engine")]
    engine: String,
    #[structopt(
        long = "addr",
        help = "Sets the server address",
        parse(try_from_str)
    )]
    socket: SocketAddr,
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}

fn main() -> Result<()> {
    let mut store = KvStore::open(current_dir()?)?;
    let config = KvsServer::from_args();
    let mut exit_code = 0;

    exit(exit_code)
}
