#[macro_use(load_yaml)]
extern crate clap;
use clap::App;
use kvs::KvStore;
use std::process::exit;

fn main() {
    let yaml = load_yaml!("./cli.yml");
    let app = App::from_yaml(yaml)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"));
    let arg_matches = app.get_matches();
    let (subcommand, _subcommand_matches) = arg_matches.subcommand();

    match subcommand {
        "get" => {
            eprintln!("unimplemented");
            exit(123);
        },
        "set" => {
            eprintln!("unimplemented");
            exit(123);
        },
        "rm" => {
            eprintln!("unimplemented");
            exit(123);
        },
        _ => panic!("Unrecognized argument."),
    }
}