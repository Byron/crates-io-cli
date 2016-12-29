use futures_cpupool::CpuPool;
use futures::Future;
use std::time::Duration;
use std::io::{self, Write};
use std::fmt::{self, Formatter, Display};
use rustc_serialize::Encodable;
use rustc_serialize::json;
use tokio_core::reactor::{Timeout, Core};
use tokio_core;

use clap;
use std;
use crates_index_diff::{CrateVersion, Index};
use utils::ok_or_exit;

arg_enum!{
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum OutputKind {
        human,
        json
    }
}

struct ForHumans<'a>(&'a CrateVersion);

impl<'a> Display for ForHumans<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.0.name, self.0.version, self.0.kind)
    }
}

pub fn handle_recent_changes(repo_path: &str, args: &clap::ArgMatches) {
    let mut reactor = ok_or_exit(Core::new());
    let handle: tokio_core::reactor::Handle = reactor.handle();
    let timeout: Timeout = ok_or_exit(Timeout::new(Duration::from_secs(3), &handle));
    let pool = CpuPool::new(1);

    let output_kind: OutputKind =
        args.value_of("format").expect("default to be set").parse().expect("clap to work");
    let owned_repo_path = repo_path.to_owned();

    let computation = pool.spawn_fn(move || {
        ok_or_exit(std::fs::create_dir_all(&owned_repo_path));
        let index = ok_or_exit(Index::from_path_or_cloned(owned_repo_path));
        let stdout = io::stdout();
        let mut channel = stdout.lock();
        let changes = ok_or_exit(index.fetch_changes());

        match output_kind {
            OutputKind::human => {
                for version in changes {
                    writeln!(channel, "{}", ForHumans(&version)).ok();
                }
            }
            OutputKind::json => {
                let mut buf = String::with_capacity(256);
                for version in changes {
                    buf.clear();
                    // unfortunately io::Write cannot be used directly, the encoder needs fmt::Write
                    // To allow us reusing the buffer, we need to restrict its lifetime.
                    if {
                            let mut encoder = json::Encoder::new(&mut buf);
                            version.encode(&mut encoder)
                        }
                        .is_ok() {
                        writeln!(channel, "{}", buf).ok();
                    }
                }
            }
        }
        Ok(Ok(()))
    });
    let owned_repo_path = repo_path.to_owned();
    let timeout = timeout.and_then(move |_| {
        writeln!(std::io::stderr(),
                 "Please wait while we check out or fetch the crates.io index at \
                  '{path}'",
                 path = owned_repo_path)
            .ok();
        Ok(Err(()))
    });
    let computation = computation.select(timeout).then(|res| {
        match res {
            Ok((Ok(_), _)) => Ok(()),
            Ok((Err(_), computation)) => computation.wait().map(|_| ()),
            Err((e, _drop_timeout)) => Err(e),
        }
    });
    ok_or_exit(reactor.run(computation));
}
