extern crate kvs;

use kvs::Result;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Config {
    Get { key: String },
    Set { key: String, value: String },
    Rm { key: String },
}
fn main() -> Result<()> {
    match Config::from_args() {
        Config::Get { key: _ } => {
            eprintln!("unimplemented");
        }
        Config::Set { key: _, value: _ } => {
            eprintln!("unimplemented");
        }
        Config::Rm { key: _ } => {
            eprintln!("unimplemented");
        }
    }
    Ok(())
}
