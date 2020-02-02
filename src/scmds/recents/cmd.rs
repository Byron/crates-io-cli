use super::error::Error;
use crate::structs::OutputKind;
use futures::Future;
use futures_cpupool::CpuPool;
use std::{self, env, io::Write, path::PathBuf, time::Duration};
use tokio_core::reactor::{Core, Timeout};

use crate::utils::json_to_stdout;
use crates_index_diff::Index;
use prettytable::{format, Table};
use tokio_core;

enum ResultKind {
    ComputationDone,
    Timeout,
}

fn show_changes(repo_path: PathBuf, output_kind: OutputKind) -> Result<ResultKind, Error> {
    std::fs::create_dir_all(&repo_path)
        .map_err(|e| Error::RepositoryDirectory(e, repo_path.clone().into()))?;
    let index = Index::from_path_or_cloned(repo_path)?;
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
        OutputKind::json => json_to_stdout(&changes),
    }
    Ok(ResultKind::ComputationDone)
}

fn default_repository_dir() -> PathBuf {
    let mut p = env::temp_dir();
    p.push("crates-io-bare-clone_for-cli");
    p
}

pub fn handle_recent_changes(
    repo_path: Option<PathBuf>,
    output_format: OutputKind,
) -> Result<(), Error> {
    let mut reactor = Core::new().map_err(Error::ReactorInit)?;
    let handle: tokio_core::reactor::Handle = reactor.handle();
    let timeout: Timeout = Timeout::new(Duration::from_secs(3), &handle).map_err(Error::Timeout)?;
    let pool = CpuPool::new(1);

    let repo_path = repo_path.unwrap_or_else(default_repository_dir);

    let computation = {
        let repo_path = repo_path.clone();
        pool.spawn_fn(move || show_changes(repo_path, output_format))
    };
    let timeout = timeout
        .map(move |_| {
            writeln!(
                std::io::stderr(),
                "Please wait while we check out or fetch the crates.io index at '{path}'",
                path = repo_path.display()
            )
            .ok();
            ResultKind::Timeout
        })
        .map_err(Error::Timeout);
    let computation = computation.select(timeout).then(|res| match res {
        Ok((ResultKind::ComputationDone, _)) => Ok(()),
        Ok((ResultKind::Timeout, computation)) => computation.wait().map(|_| ()),
        Err((e, _drop_timeout)) => Err(e),
    });
    reactor.run(computation)
}
