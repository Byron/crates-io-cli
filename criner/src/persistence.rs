use crate::{
    error::{Error, Result},
    model::Crate,
};
use crates_index_diff::CrateVersion;
use serde_derive::{Deserialize, Serialize};
use sled::IVec;
use std::time::SystemTime;
use std::{path::Path, time};

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
    pub fetch_crate_versions: time::Duration,
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
    pub sample_time: time::SystemTime,
    pub delta: Context,
}

impl From<ContextDeltaVec> for IVec {
    fn from(c: ContextDeltaVec) -> Self {
        rmp_serde::to_vec(&c)
            .expect("serialization must never fail")
            .into()
    }
}

impl From<IVec> for ContextDeltaVec {
    fn from(v: IVec) -> Self {
        rmp_serde::from_read(v.as_ref()).expect("always valid decoding: TODO: migrations")
    }
}

impl From<&[u8]> for ContextDeltaVec {
    fn from(v: &[u8]) -> Self {
        rmp_serde::from_read(v).expect("always valid decoding: TODO: migrations")
    }
}

/// This structure is just for serialization
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextDeltaVec(Vec<ContextDelta>);

impl From<(time::SystemTime, &Context, &Context)> for ContextDelta {
    fn from((sample_time, now, earlier): (SystemTime, &Context, &Context)) -> Self {
        ContextDelta {
            sample_time,
            delta: now.change_since(earlier),
        }
    }
}

impl From<Context> for IVec {
    fn from(c: Context) -> Self {
        rmp_serde::to_vec(&c)
            .expect("serialization must never fail")
            .into()
    }
}

impl From<IVec> for Context {
    fn from(v: IVec) -> Self {
        rmp_serde::from_read(v.as_ref()).expect("always valid decoding: TODO: migrations")
    }
}

impl From<&[u8]> for Context {
    fn from(v: &[u8]) -> Self {
        rmp_serde::from_read(v).expect("always valid decoding: TODO: migrations")
    }
}

#[derive(Clone)]
pub struct Db {
    inner: sled::Db,
    meta: sled::Tree,
}

impl Db {
    pub fn open(path: impl AsRef<Path>) -> Result<Db> {
        let inner = sled::open(path)?;
        let meta = inner.open_tree("meta")?;
        Ok(Db { inner, meta })
    }

    pub fn open_crate_versions(&self) -> Result<CrateVersionsTree> {
        Ok(CrateVersionsTree {
            inner: self.inner.open_tree("crate_versions")?,
        })
    }

    pub fn open_crates(&self) -> Result<CratesTree> {
        Ok(CratesTree {
            inner: self.inner.open_tree("crates")?,
        })
    }

    const CONTEXT_GLOBAL: &'static [u8] = b"context";
    const CONTEXT_SERIES_PREFIX: &'static str = "context/";

    pub fn update_context(&self, f: impl Fn(&mut Context)) -> Result<Context> {
        self.meta
            .update_and_fetch(Self::CONTEXT_GLOBAL, |bytes: Option<&[u8]>| {
                Some(match bytes {
                    Some(bytes) => {
                        // NOTE: We assume that a version can only be added once! They are immutable.
                        let mut ctx = bytes.into();
                        f(&mut ctx);
                        ctx
                    }
                    None => Context::default(),
                })
            })?
            .map(From::from)
            .ok_or_else(|| Error::Bug("We always set a context"))
    }

    pub fn context(&self) -> Result<Context> {
        self.meta
            .get(Self::CONTEXT_GLOBAL)
            .map_err(From::from)
            .map(|bytes| bytes.map_or_else(Context::default, From::from))
    }

    pub fn insert_context_delta(&self, earlier: Context) -> Result<Vec<ContextDelta>> {
        assert_eq!(
            Self::CONTEXT_GLOBAL,
            &Self::CONTEXT_SERIES_PREFIX.as_bytes()[..Self::CONTEXT_GLOBAL.len()]
        );
        assert_eq!(
            Self::CONTEXT_SERIES_PREFIX.len() - 1,
            Self::CONTEXT_GLOBAL.len()
        );

        let sample_time = SystemTime::now();
        let key = format!(
            "{}{}",
            Self::CONTEXT_SERIES_PREFIX,
            humantime::format_rfc3339(sample_time)
                .to_string()
                .get(..10)
                .expect("YYYY-MM-DD - 10 bytes")
        );
        let current: Context = self.context()?;
        self.meta
            .update_and_fetch(key, move |bytes: Option<&[u8]>| {
                let delta: ContextDelta = (sample_time, &current, &earlier).into();
                Some(match bytes {
                    Some(bytes) => {
                        let mut samples: ContextDeltaVec = rmp_serde::from_read(bytes).expect(
                            "deserialization of ContextDelta must not fail. Migration TODO",
                        );
                        samples.0.push(delta);
                        samples
                    }
                    None => ContextDeltaVec(vec![delta]),
                })
            })?
            .map(From::from)
            .map(|ContextDeltaVec(v)| v)
            .ok_or_else(|| Error::Bug("We always have at least one sample"))
    }
}

fn version_id(v: &CrateVersion) -> Vec<u8> {
    let mut id = Vec::with_capacity(v.name.len() + v.version.len() + 1);
    id.extend_from_slice(&v.name.as_bytes());
    id.push(b':');
    id.extend_from_slice(&v.version.as_bytes());
    id
}

pub fn crate_name(v: &CrateVersion) -> Vec<u8> {
    v.name.clone().into_bytes()
}

#[derive(Clone)]
pub struct CratesTree {
    inner: sled::Tree,
}

impl From<Crate> for IVec {
    fn from(c: Crate) -> Self {
        rmp_serde::to_vec(&c)
            .expect("serialization to always succeed")
            .into()
    }
}

impl From<&[u8]> for Crate {
    fn from(b: &[u8]) -> Self {
        rmp_serde::from_read(b).expect("always valid decoding: TODO: migrations")
    }
}

impl From<IVec> for Crate {
    fn from(b: IVec) -> Self {
        rmp_serde::from_read(b.as_ref()).expect("always valid decoding: TODO: migrations")
    }
}

impl CratesTree {
    pub fn insert_version(&self, v: &CrateVersion) -> Result<bool> {
        self.inner
            .update_and_fetch(crate_name(v), |bytes: Option<&[u8]>| {
                Some(match bytes {
                    Some(bytes) => {
                        let mut c = Crate::from(bytes);
                        c.versions.push(v.version.to_owned());
                        c.versions.sort();
                        c
                    }
                    None => Crate {
                        versions: vec![v.version.to_owned()],
                    },
                })
            })?
            .ok_or_else(|| Error::Bug("We always put a crate or update the existing one"))
            .map(Crate::from)
            .map(|c| c.versions.len() == 1)
    }
}

#[derive(Clone)]
pub struct CrateVersionsTree {
    inner: sled::Tree,
}

impl CrateVersionsTree {
    pub fn insert(&self, v: &CrateVersion) -> Result<()> {
        self.inner
            .insert(version_id(v), rmp_serde::to_vec(v)?)
            .map_err(Error::from)
            .map(|_| ())
    }
}
