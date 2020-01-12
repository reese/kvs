use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Config {
    Get {
        key: String
    },
    Set {
        key: String,
        value: String
    },
    Rm {
        key: String
    }
}

fn main() {
    match Config::from_args() {
        Config::Get { key: _ } => {
            eprintln!("unimplemented");
            exit(123);
        }
        Config::Set { key: _, value: _ } => {
            eprintln!("unimplemented");
            exit(123);
        }
        Config::Rm { key: _ } => {
            eprintln!("unimplemented");
            exit(123);
        }
    }
}
