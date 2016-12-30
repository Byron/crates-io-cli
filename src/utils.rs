use std::error::Error;
use std::default::Default;

use std;

#[derive(RustcDecodable, Clone)]
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
        #[cfg(windows)]
        fn imp() -> Dimension {
            Dimension {
                width: 80,
                height: 20,
            }
        }

        #[cfg(unix)]
        fn imp() -> Dimension {
            use termion::terminal_size;
            let (mw, mh) = terminal_size().unwrap_or((80, 20));
            Dimension {
                width: mw,
                height: mh,
            }
        }

        imp()
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
