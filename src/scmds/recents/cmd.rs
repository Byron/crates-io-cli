use super::error::Error;
use crate::args::OutputKind;
use std::{
    env,
    io::{self, Write},
    ops::Add,
    path::PathBuf,
    time::Duration,
};

use crates_index_diff::Index;
use parking_lot::{Condvar, Mutex};
use prettytable::{format, Table};

fn show_changes(repo_path: PathBuf, output_kind: OutputKind) -> Result<(), Error> {
    std::fs::create_dir_all(&repo_path).map_err(|e| Error::RepositoryDirectory {
        source: e,
        path: repo_path.clone().into(),
    })?;
    let index = Index::from_path_or_cloned(repo_path)?;
    let changes = index.fetch_changes()?;

    match output_kind {
        OutputKind::Human => {
            if !changes.is_empty() {
                let table = {
                    let mut t = Table::new();
                    t.set_titles(row![b -> "Name", b -> "Version", b -> "Kind"]);
                    t.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                    changes.iter().fold(t, |mut t, c| {
                        let v = &c.versions()[0];
                        t.add_row(row![v.name, v.version, c.to_string()]);
                        t
                    })
                };
                table.print_tty(false)?;
            }
            Ok(())
        }
        OutputKind::Json => {
            let versions: Vec<_> = changes.iter().map(|c| &c.versions()[0]).collect();
            serde_json::to_writer_pretty(io::stdout(), &versions).map_err(Into::into)
        }
    }
}

fn default_repository_dir() -> PathBuf {
    let mut p = env::temp_dir();
    p.push("crates-io-bare-clone_for-cli");
    p
}

fn message_after_timeout(msg: String, d: Duration) {
    std::thread::spawn({
        move || {
            let lock = Mutex::new(());
            let cvar = Condvar::new();
            let mut something_to_wait_on = lock.lock();
            if cvar
                .wait_until(&mut something_to_wait_on, std::time::Instant::now().add(d))
                .timed_out()
            {
                writeln!(std::io::stderr(), "{}", msg).ok();
            }
        }
    });
}

pub fn handle_recent_changes(
    repo_path: Option<PathBuf>,
    output_format: OutputKind,
) -> Result<(), Error> {
    let repo_path = repo_path.unwrap_or_else(default_repository_dir);
    let computation = std::thread::spawn({
        let repo_path = repo_path.clone();
        move || show_changes(repo_path, output_format)
    });
    message_after_timeout(
        format!(
            "Please wait while we check out or fetch the crates.io index at '{path}'",
            path = repo_path.display()
        ),
        Duration::from_secs(3),
    );
    computation.join().unwrap()
}
