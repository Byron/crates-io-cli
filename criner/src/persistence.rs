use crate::{
    error::{Error, Result},
    model::Crate,
};
use crates_index_diff::CrateVersion;
use serde_derive::{Deserialize, Serialize};
use sled::IVec;
use std::path::Path;

/// Stores information about the work we have performed thus far
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Context {
    /// The amount of crate versions stored in the database
    pub num_crate_versions: u64,

    /// The amount of crates in the database
    pub num_crates: u32,
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

    pub fn update_context(&self, f: impl Fn(&mut Context)) -> Result<Context> {
        self.meta
            .update_and_fetch(b"context", |bytes: Option<&[u8]>| {
                let ctx = match bytes {
                    Some(bytes) => {
                        let mut ctx = bytes.into();
                        f(&mut ctx);
                        ctx
                    }
                    None => Context::default(),
                };
                Some(ctx)
            })?
            .map(From::from)
            .ok_or_else(|| Error::Bug("We always set a context"))
    }

    pub fn context(&self) -> Result<Context> {
        self.meta
            .get(b"context")
            .map_err(From::from)
            .and_then(|bytes| {
                bytes
                    .ok_or_else(|| Error::Bug("the context has been updated at least once"))
                    .map(From::from)
            })
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
