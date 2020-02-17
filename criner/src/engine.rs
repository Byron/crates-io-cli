use crate::{
    error::{Error, Result},
    model,
    persistence::{Db, TreeAccess},
    utils::*,
};
use crates_index_diff::Index;
use futures::future::Either;
use futures::{
    executor::{block_on, ThreadPool},
    future::FutureExt,
    stream::StreamExt,
    task::{Spawn, SpawnExt},
};
use log::info;
use prodash::tui::{Event, Line};
use std::{
    io::Write,
    path::Path,
    path::PathBuf,
    time::{Duration, SystemTime},
};

async fn process_changes(
    db: Db,
    crates_io_path: impl AsRef<Path>,
    deadline: Option<SystemTime>,
    pool: impl Spawn,
    mut progress: prodash::tree::Item,
) -> Result<()> {
    let start = SystemTime::now();
    progress.info("Potentially cloning crates index - this can take a while…");
    let index = enforce_blocking(
        deadline,
        {
            let path = crates_io_path.as_ref().to_path_buf();
            || Index::from_path_or_cloned(path)
        },
        &pool,
    )
    .await??;
    progress.info("Fetching crates index to see changes");
    let crate_versions = enforce_blocking(deadline, move || index.fetch_changes(), &pool).await??;

    progress.done(format!("Fetched {} changed crates", crate_versions.len()));
    let mut store_progress = progress.add_child("processing new crates");
    store_progress.init(Some(crate_versions.len() as u32), Some("crates"));

    enforce_blocking(
        deadline,
        {
            let db = db.clone();
            move || {
                let versions = db.open_crate_versions()?;
                let krate = db.open_crates()?;
                let context = db.context();
                // NOTE: this loop can also be a stream, but that makes computation slower due to overhead
                // Thus we just do this 'quickly' on the main thread, knowing that criner really needs its
                // own executor or resources.
                // We could chunk things, but that would only make the code harder to read. No gains here…
                // NOTE: Even chunks of 1000 were not faster, didn't even saturate a single core...
                for (versions_stored, version) in crate_versions.iter().enumerate() {
                    // NOTE: For now, not transactional, but we *could*!
                    {
                        versions.insert(&version)?;
                        context.update_today(|c| c.counts.crate_versions += 1)?;
                    }
                    if krate.upsert(&version)? {
                        context.update_today(|c| c.counts.crates += 1)?;
                    }
                    store_progress.set((versions_stored + 1) as u32);
                }
                context.update_today(|c| {
                    c.durations.fetch_crate_versions += SystemTime::now()
                        .duration_since(start)
                        .unwrap_or_else(|_| Duration::default())
                })?;
                Ok::<_, Error>(())
            }
        },
        &pool,
    )
    .await??;
    Ok(())
}

/// Runs the statistics and mining engine.
/// May run for a long time unless a deadline is specified.
/// Even though timeouts can be achieved from outside of the future, knowing the deadline may be used
/// by the engine to manage its time even more efficiently.
pub async fn run(
    db: Db,
    crates_io_path: PathBuf,
    deadline: Option<SystemTime>,
    progress: prodash::Tree,
    pool: impl Spawn,
) -> Result<()> {
    check(deadline)?;

    let res = {
        let db = db.clone();
        process_changes(
            db,
            crates_io_path,
            deadline,
            pool,
            progress.add_child("crates.io refresh"),
        )
        .await
    };

    res
}

/// For convenience, run the engine and block until done.
pub fn run_blocking(
    db: impl AsRef<Path>,
    crates_io_path: impl AsRef<Path>,
    deadline: Option<SystemTime>,
) -> Result<()> {
    let start_of_computation = SystemTime::now();
    // NOTE: pool should be big enough to hold all possible blocking tasks running in parallel.
    // The main thread is expected to pool non-blocking tasks.
    // Of course, non-blocking tasks may also be scheduled there, which is when you probably want
    // to have another free thread just for that.
    // All this is theory.
    let pool_size = 2;
    let blocking_task_pool = ThreadPool::builder().pool_size(pool_size).create()?;
    let db = Db::open(db)?;

    let root = prodash::Tree::new();
    let (gui, abort_handle) = futures::future::abortable(prodash::tui::render_with_input(
        root.clone(),
        prodash::tui::TuiOptions {
            title: "Criner".into(),
            ..prodash::tui::TuiOptions::default()
        },
        context_stream(&db, start_of_computation),
    )?);

    // dropping the work handle will stop (non-blocking) futures
    let _work_handle = blocking_task_pool.spawn_with_handle(
        run(
            db.clone(),
            crates_io_path.as_ref().into(),
            deadline,
            root,
            blocking_task_pool.clone(),
        )
        .map(|_| ()),
    )?;

    let either = block_on(futures::future::select(_work_handle, gui.boxed_local()));
    match either {
        Either::Left((_, gui)) => {
            abort_handle.abort();
            block_on(gui).ok();
        }
        Either::Right((_, work_handle)) => work_handle.forget(),
    }

    // Make sure the terminal can reset when the gui is done.
    std::io::stdout().flush()?;

    // at this point, we forget all currently running computation, and since it's in the local thread, it's all
    // destroyed/dropped properly.
    info!("{}", wallclock(start_of_computation));
    Ok(())
}

fn wallclock(since: SystemTime) -> String {
    format!(
        "Wallclock elapsed: {}",
        humantime::format_duration(SystemTime::now().duration_since(since).unwrap_or_default())
    )
}

fn context_stream(db: &Db, start_of_computation: SystemTime) -> impl futures::Stream<Item = Event> {
    prodash::tui::ticker(Duration::from_secs(1)).map({
        let db = db.clone();
        move |_| {
            db.context()
                .iter()
                .next_back()
                .and_then(Result::ok)
                .map(|(_, c): (_, model::Context)| {
                    let lines = vec![
                        Line::Text(wallclock(start_of_computation)),
                        Line::Title("Durations".into()),
                        Line::Text(format!(
                            "fetch-crate-versions: {:?}",
                            c.durations.fetch_crate_versions
                        )),
                        Line::Title("Counts".into()),
                        Line::Text(format!("crate-versions: {}", c.counts.crate_versions)),
                        Line::Text(format!("        crates: {}", c.counts.crates)),
                    ];
                    Event::SetInformation(lines)
                })
                .unwrap_or(Event::Tick)
        }
    })
}
