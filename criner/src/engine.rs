use crate::persistence::CrateVersionsTree;
use crate::{
    error::{Error, Result},
    model,
    persistence::{Db, TreeAccess},
    utils::*,
};
use crates_index_diff::Index;
use futures::{
    executor::{block_on, ThreadPool},
    future::Either,
    future::FutureExt,
    stream::StreamExt,
    task::{Spawn, SpawnExt},
};
use futures_timer::Delay;
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
    _local: impl Spawn,
    blocking: impl Spawn,
    mut progress: prodash::tree::Item,
) -> Result<()> {
    let start = SystemTime::now();
    let mut subprogress =
        progress.add_child("Potentially cloning crates index - this can take a while…");
    let index = enforce_blocking(
        deadline,
        {
            let path = crates_io_path.as_ref().to_path_buf();
            || Index::from_path_or_cloned(path)
        },
        &blocking,
    )
    .await??;
    subprogress.set_name("Fetching crates index to see changes");
    let crate_versions =
        enforce_blocking(deadline, move || index.fetch_changes(), &blocking).await??;

    progress.done(format!("Fetched {} changed crates", crate_versions.len()));
    drop(subprogress);

    let mut store_progress = progress.add_child("processing new crates");
    store_progress.init(Some(crate_versions.len() as u32), Some("crate versions"));

    enforce_future(
        deadline,
        {
            let db = db.clone();
            async move {
                let versions = db.open_crate_versions()?;
                let krate = db.open_crates()?;
                let context = db.context();
                let mut may_schedule_tasks = true;
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

                    // There is enough scheduling capacity for this not to block
                    // TODO: one day we may decide based on other context whether to continue
                    // blocking while trying, or not, or try again a bit later after storing
                    // a chunk of versions
                    if may_schedule_tasks {
                        let res = schedule_tasks(
                                version,
                                store_progress.add_child(format!("schedule {}", CrateVersionsTree::key_str(version))),
                                Scheduling::NeverBlock,
                            )
                            .await?;
                        if let AsyncResult::WouldBlock  = res {
                            store_progress.info("Skipping further task scheduling in preference for storing new versions");
                            may_schedule_tasks = false;
                        }
                    }
                    store_progress.set((versions_stored + 1) as u32);
                }
                context.update_today(|c| {
                    c.durations.fetch_crate_versions += SystemTime::now()
                        .duration_since(start)
                        .unwrap_or_else(|_| Duration::default())
                })?;
                store_progress.done(format!(
                    "Stored {} crate versions to database",
                    crate_versions.len()
                ));
                Ok::<_, Error>(())
            }
        },
        &blocking,
    )
    .await??;
    Ok(())
}

enum Scheduling {
    //   /// Considers work done if everything was done. Will block to assure that
    //    All,
    //    /// Considers the work done if at least one task was scheduled. Will block to wait otherwise.
    //    AtLeastOne,
    /// Prefer to never wait for workers to perform a task and instead return without having scheduled anything
    NeverBlock,
}

enum AsyncResult {
    /// The required scheduling cannot be fulfilled without blocking
    WouldBlock,
    /// The minimal scheduling requirement was met
    Done,
}

async fn schedule_tasks(
    _version: &crates_index_diff::CrateVersion,
    mut progress: prodash::tree::Item,
    _mode: Scheduling,
) -> Result<AsyncResult> {
    progress.init(None, Some("tasks"));
    Ok(AsyncResult::WouldBlock)
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
    local: impl Spawn,
    blocking: impl Spawn,
) -> Result<()> {
    check(deadline)?;

    let mut downloaders = progress.add_child("Downloads");
    for idx in 0..10 {
        blocking.spawn({
            let mut progress = downloaders.add_child(format!("DL {} - idle", idx + 1));
            async move {
                let mut iteration = 0;
                progress.init(None, Some("Kb"));
                loop {
                    iteration += 1;
                    Delay::new(Duration::from_secs(1)).await;
                    progress.set(iteration)
                }
            }
        })?;
    }

    let res = {
        let db = db.clone();
        process_changes(
            db,
            crates_io_path,
            deadline,
            blocking,
            local,
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
    let pool_size = 1;
    let blocking_task_pool = ThreadPool::builder().pool_size(pool_size).create()?;
    // Non-blocking tasks should be scheduled here - we can't use the local pool for this, as its spawner
    // cannot be sent across threads, and thus doesn't work in 'run'
    let task_pool = ThreadPool::builder().pool_size(1).create()?;
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
    let work_handle = task_pool.spawn_with_handle(run(
        db.clone(),
        crates_io_path.as_ref().into(),
        deadline,
        root,
        task_pool.clone(),
        blocking_task_pool.clone(),
    ))?;

    let either = block_on(futures::future::select(work_handle, gui.boxed_local()));
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
