use crates_index_diff::git2;
use humantime;
use rmp_serde;
use sled;
use std::fmt;
use std::time;

#[derive(Debug)]
pub struct DeadlineFormat(pub time::SystemTime);

impl fmt::Display for DeadlineFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        let now = time::SystemTime::now();
        write!(
            f,
            "{} ago at {}",
            humantime::format_duration(now.duration_since(self.0).unwrap_or_default()),
            humantime::format_rfc3339(now)
        )
    }
}

pub type Result<T> = std::result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Bug(d: &'static str) {
            display("{}", d)
        }
        DeadlineExceeded(d: DeadlineFormat) {
            display("Stopped computation as deadline was reached {}.", d)
        }
        RmpSerdeEncode(err: rmp_serde::encode::Error) {
            from()
            cause(err)
        }
        Git2(err: git2::Error) {
            from()
            cause(err)
        }
        Sled(err: sled::Error) {
            from()
            cause(err)
        }
    }
}
