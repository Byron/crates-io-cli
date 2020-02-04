use serde_derive::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Represents a top-level crate and associated information
#[derive(Serialize, Deserialize)]
pub struct Crate {
    /// All versions published to crates.io, guaranteed to be sorted so that the most recent version is last.
    /// The format is as specified in Cargo.toml:version
    pub versions: Vec<String>,
}

/// Stores element counts of various kinds
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Counts {
    /// The amount of crate versions stored in the database
    pub crate_versions: u64,

    /// The amount of crates in the database
    pub crates: u32,
}

/// Stores wall clock time that elapsed for various kinds of computation
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Durations {
    pub fetch_crate_versions: Duration,
}

/// Stores information about the work we have performed thus far
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Context {
    /// Various elements counts
    pub counts: Counts,
    /// Various kinds of time we took for computation
    pub durations: Durations,
}

impl Context {
    fn change_since(&self, earlier: &Context) -> Context {
        Context {
            counts: Counts {
                crate_versions: self.counts.crate_versions - earlier.counts.crate_versions,
                crates: self.counts.crates - earlier.counts.crates,
            },
            durations: Durations {
                fetch_crate_versions: self.durations.fetch_crate_versions
                    - earlier.durations.fetch_crate_versions,
            },
        }
    }
}

/// Represents the difference between a current context and an earlier one, at a time
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextDelta {
    pub sample_time: SystemTime,
    pub delta: Context,
}

/// This structure is just for serialization
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextDeltaVec(pub Vec<ContextDelta>);

impl From<(SystemTime, &Context, &Context)> for ContextDelta {
    fn from((sample_time, now, earlier): (SystemTime, &Context, &Context)) -> Self {
        ContextDelta {
            sample_time,
            delta: now.change_since(earlier),
        }
    }
}
