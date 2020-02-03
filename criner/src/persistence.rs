use crate::{
    error::{Error, Result},
    model::Crate,
};
use crates_index_diff::CrateVersion;
use sled::IVec;
use std::path::Path;

#[derive(Clone)]
pub struct Db {
    inner: sled::Db,
}

impl Db {
    pub fn open(path: impl AsRef<Path>) -> Result<Db> {
        Ok(Db {
            inner: sled::open(path)?,
        })
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
    pub fn insert_version(&self, v: &CrateVersion) -> Result<Crate> {
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
            .map(Crate::from)
            .ok_or_else(|| Error::Bug("We should always insert a crate, but didn't get any"))
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
