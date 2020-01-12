#[macro_use(load_yaml)]
extern crate clap;
use clap::App;
use kvs::KvStore;
use std::process::exit;

fn main() {
    let yaml = load_yaml!("./cli.yml");
    let arg_matches = App::from_yaml(yaml).get_matches();

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