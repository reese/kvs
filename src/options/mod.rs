use std::net::SocketAddr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Options {
    #[structopt(
        long = "addr",
        help = "Sets the server address",
        default_value = "127.0.0.1:4000",
        parse(try_from_str)
    )]
    pub socket: SocketAddr,
    #[structopt(default_value = "kvs", long = "engine")]
    pub engine: String,
}
