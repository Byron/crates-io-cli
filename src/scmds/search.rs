extern crate termion;

use clap;
use self::termion::event::Key;
use self::termion::raw::IntoRawMode;
use self::termion::input::TermRead;
use std::io::{self, Write};

use utils::ok_or_exit;

pub fn handle_interactive_search(_args: &clap::ArgMatches) {
    let stdin = io::stdin();
    let mut stdout = ok_or_exit(io::stdout().into_raw_mode());
    for k in stdin.keys() {
        match ok_or_exit(k) {
            Key::Char('q') => break,
            Key::Char(c) => print!("{}", c),
            _ => println!("unsupported!"),
        }
        stdout.flush().ok();
    }
}
