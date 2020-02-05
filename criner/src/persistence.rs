use crate::{
    error::{Error, Result},
    model::{Context, ContextDelta, ContextDeltaVec, Crate},
};
use crates_index_diff::CrateVersion;
use sled::{IVec, Tree};
use std::path::Path;
use std::time::SystemTime;

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

#[derive(Clone)]
pub struct CratesTree {
    inner: sled::Tree,
}

pub trait TreeAccess {
    type Item;
    type InsertResult;

    fn tree(&self) -> &sled::Tree;

    fn key(&self, item: &Self::Item) -> Vec<u8>;
    fn map_insert_return_value(&self, v: IVec) -> Self::InsertResult;
    fn merge(&self, new_item: &Self::Item, existing_item: Option<&[u8]>) -> Option<IVec>;

    fn insert(&self, item: &Self::Item) -> Result<Self::InsertResult> {
        self.tree()
            .update_and_fetch(self.key(item), |bytes: Option<&[u8]>| {
                self.merge(item, bytes)
            })?
            .ok_or_else(|| Error::Bug("We always put a crate or update the existing one"))
            .map(|v| self.map_insert_return_value(v))
    }
}

impl TreeAccess for CratesTree {
    type Item = CrateVersion;
    type InsertResult = bool;

    fn tree(&self) -> &Tree {
        &self.inner
    }

    fn key(&self, item: &CrateVersion) -> Vec<u8> {
        item.name.clone().into_bytes()
    }

    fn map_insert_return_value(&self, v: IVec) -> Self::InsertResult {
        let c = Crate::from(v);
        c.versions.len() == 1
    }

    fn merge(&self, new_item: &CrateVersion, existing_item: Option<&[u8]>) -> Option<IVec> {
        Some(match existing_item {
            Some(bytes) => {
                let mut c = Crate::from(bytes);
                c.versions.push(new_item.version.to_owned());
                c.versions.sort();
                c
            }
            None => Crate {
                versions: vec![new_item.version.to_owned()],
            },
        })
        .map(IVec::from)
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

macro_rules! impl_ivec_transform {
    ($ty:ty) => {
        impl From<&[u8]> for $ty {
            fn from(b: &[u8]) -> Self {
                rmp_serde::from_read(b).expect("always valid decoding: TODO: migrations")
            }
        }
        impl From<IVec> for $ty {
            fn from(b: IVec) -> Self {
                rmp_serde::from_read(b.as_ref()).expect("always valid decoding: TODO: migrations")
            }
        }
        impl From<$ty> for IVec {
            fn from(c: $ty) -> Self {
                rmp_serde::to_vec(&c)
                    .expect("serialization to always succeed")
                    .into()
            }
        }
    };
}

impl_ivec_transform!(Crate);
impl_ivec_transform!(Context);
impl_ivec_transform!(ContextDeltaVec);
