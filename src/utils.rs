use std::error::Error;
use std::default::Default;

use std;
use termion::terminal_size;

#[derive(RustcDecodable)]
pub struct Dimension {
    pub width: u16,
    pub height: u16,
}

impl Dimension {
    pub fn loose_heigth(mut self, h: u16) -> Dimension {
        self.height -= h;
        self
    }
}

impl Default for Dimension {
    fn default() -> Dimension {
        let (mw, mh) = terminal_size().unwrap_or((80, 20));
        Dimension {
            width: mw,
            height: mh,
        }
    }
}

pub fn ok_or_exit<T, E>(result: Result<T, E>) -> T
    where E: Error
{
    match result {
        Ok(v) => v,
        Err(err) => {
            println!("{}", err);
            std::process::exit(2);
        }
    }
}
