extern crate termion;

use clap;
use self::termion::event::Key;
use self::termion::raw::IntoRawMode;
use self::termion::input::TermRead;
use self::termion::clear;
use self::termion::cursor;
use std::io::{self, Write};

use utils::ok_or_exit;

pub fn handle_interactive_search(_args: &clap::ArgMatches) {
    let stdin = io::stdin();
    let mut stdout = ok_or_exit(io::stdout().into_raw_mode());
    let mut term = String::new();
    for k in stdin.keys() {
        match ok_or_exit(k) {
            Key::Char(c) => {
              term.push(c);
            },
            Key::Backspace => {
                term.pop();
            }
            _ => println!("unsupported!"),
        }
        write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), term).unwrap();
        stdout.flush().ok();
    }
}
