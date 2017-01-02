use super::error::Error;
use futures_cpupool::CpuPool;
use futures::Future;
use std::time::Duration;
use std::io::{self, Write};
use rustc_serialize::{json, Encodable};
use tokio_core::reactor::{Timeout, Core};

use clap;
use prettytable::format;
use prettytable::Table;
use std;
use tokio_core;
use crates_index_diff::Index;

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum OutputKind {
        human,
        json
    }
}

pub fn handle_recent_changes(repo_path: &str, args: &clap::ArgMatches) -> Result<(), Error> {
    let mut reactor = Core::new().map_err(Error::ReactorInit)?;
    let handle: tokio_core::reactor::Handle = reactor.handle();
    let timeout: Timeout = Timeout::new(Duration::from_secs(3), &handle).map_err(Error::Timeout)?;
    let pool = CpuPool::new(1);

    let output_kind: OutputKind =
        args.value_of("format").expect("default to be set").parse().expect("clap to work");
    let owned_repo_path = repo_path.to_owned();

    enum Computation {
        Done,
        Timeout,
    }

    let computation = pool.spawn_fn(move || {
        std::fs::create_dir_all(&owned_repo_path)
            .map_err(|e| Error::RepositoryDirectory(e, owned_repo_path.clone().into()))?;
        let index = Index::from_path_or_cloned(owned_repo_path)?;
        let stdout = io::stdout();
        let mut channel = stdout.lock();
        let changes = index.fetch_changes()?;

        match output_kind {
            OutputKind::human => {
                if !changes.is_empty() {
                    let table = {
                        let mut t = Table::new();
                        t.set_titles(row![b -> "Name", b -> "Version", b -> "Kind"]);
                        t.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                        changes.iter().fold(t, |mut t, c| {
                            t.add_row(row![c.name, c.version, c.kind]);
                            t
                        })
                    };
                    table.print_tty(false);
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
        Ok(Computation::Done)
    });
    let owned_repo_path = repo_path.to_owned();
    let timeout = timeout.map(move |_| {
            writeln!(std::io::stderr(),
                     "Please wait while we check out or fetch the crates.io index at '{path}'",
                     path = owned_repo_path)
                .ok();
            Computation::Timeout
        })
        .map_err(Error::Timeout);
    let computation = computation.select(timeout).then(|res| {
        match res {
            Ok((Computation::Done, _)) => Ok(()),
            Ok((Computation::Timeout, computation)) => computation.wait().map(|_| ()),
            Err((e, _drop_timeout)) => Err(e),
        }
    });
    reactor.run(computation)
}
