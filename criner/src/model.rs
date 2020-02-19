use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Add;
use std::time::SystemTime;
use std::{collections::HashMap, time::Duration};

/// Represents a top-level crate and associated information
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Crate {
    /// All versions published to crates.io, guaranteed to be sorted so that the most recent version is last.
    /// The format is as specified in Cargo.toml:version
    pub versions: Vec<String>,
}

impl From<&crates_index_diff::CrateVersion> for Crate {
    fn from(v: &crates_index_diff::CrateVersion) -> Self {
        Crate {
            versions: vec![v.version.to_owned()],
        }
    }
}

/// Stores element counts of various kinds
#[derive(Default, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Counts {
    /// The amount of crate versions stored in the database
    pub crate_versions: u64,

    /// The amount of crates in the database
    pub crates: u32,
}

/// Stores wall clock time that elapsed for various kinds of computation
#[derive(Default, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Durations {
    pub fetch_crate_versions: Duration,
}

/// Stores information about the work we have performed thus far
#[derive(Default, Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Context {
    /// Various elements counts
    pub counts: Counts,
    /// Various kinds of time we took for computation
    pub durations: Durations,
}

impl Add<&Context> for Context {
    type Output = Context;

    fn add(self, rhs: &Context) -> Self::Output {
        Context {
            counts: Counts {
                crate_versions: self.counts.crate_versions + rhs.counts.crate_versions,
                crates: self.counts.crates + rhs.counts.crates,
            },
            durations: Durations {
                fetch_crate_versions: self.durations.fetch_crate_versions
                    + rhs.durations.fetch_crate_versions,
            },
        }
    }
}

/// Pack all information we know about a change made to a version of a crate.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CrateVersion {
    /// The crate name, i.e. `clap`.
    pub name: String,
    /// The kind of change.
    #[serde(rename = "yanked")]
    pub kind: crates_index_diff::ChangeKind,
    /// The semantic version of the crate.
    #[serde(rename = "vers")]
    pub version: String,
    /// The checksum over the crate archive
    #[serde(rename = "cksum")]
    pub checksum: String,
    /// All cargo features
    pub features: HashMap<String, Vec<String>>,
    /// All crate dependencies
    #[serde(rename = "deps")]
    pub dependencies: Vec<crates_index_diff::Dependency>,
}

/// Information about a process
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TaskMetaData<'a> {
    last_run_at: Option<SystemTime>,
    /// How often did we try to run the task to success
    attempts: u8,
    /// Information about the process that we used to run
    process: Cow<'a, str>,
    /// Information about the process version
    version: Cow<'a, str>,
}

/// A download with meta data and the downloaded blob itself
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Download<'a> {
    meta: TaskMetaData<'a>,
    content_length: usize,
    content_type: Option<Cow<'a, str>>,
    data: Option<Result<Cow<'a, [u8]>, Cow<'a, str>>>,
}

impl From<&crates_index_diff::CrateVersion> for CrateVersion {
    fn from(
        crates_index_diff::CrateVersion {
            name,
            kind,
            version,
            checksum,
            features,
            dependencies,
        }: &crates_index_diff::CrateVersion,
    ) -> Self {
        CrateVersion {
            name: name.clone(),
            kind: *kind,
            version: version.clone(),
            checksum: checksum.clone(),
            features: features.clone(),
            dependencies: dependencies.clone(),
        }
    }
}
