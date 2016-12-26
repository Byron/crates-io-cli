extern crate crates_index_diff;
extern crate clap;

use std::path::PathBuf;
use std::env;

use clap::{Arg, SubCommand, App};

fn default_repository_dir() -> PathBuf {
    let mut p = env::temp_dir();
    p.push("crates-io-bare-clone_for-cli");
    p
}

fn main() {
    let temp_dir = default_repository_dir();
    let temp_dir_str = temp_dir.to_string_lossy();
    let app = App::new("crates.io interface")
        .version("1.0")
        .author("Sebastian Thiel <byronimo@gmail.com>")
        .about("Interfact with the https://crates.io index via the command-line")
        .arg(Arg::with_name("repository")
            .short("r")
            .long("repository")
            .value_name("REPO")
            .help("Path to the possibly existing crates.io repository clone.")
            .default_value(&temp_dir_str)
            .required(false)
            .takes_value(true))
        .subcommand(SubCommand::with_name("recent-changes")
            .about("show all recently changed crates")
        );


    let matches = app.get_matches();
    match matches.subcommand() {
       _ => {
           print!("{}\n", matches.usage());
           std::process::exit(1);
       }
    }

}