use crate::error::{Error, Result};
use crate::model::version_id;
use crates_index_diff::CrateVersion;
use std::path::Path;

#[derive(Clone)]
pub struct Db {
    pub inner: sled::Db,
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
}

#[derive(Clone)]
pub struct CrateVersionsTree {
    pub inner: sled::Tree,
}

impl CrateVersionsTree {
    pub fn insert(&self, v: &CrateVersion) -> Result<()> {
        self.inner
            .insert(version_id(v), rmp_serde::to_vec(v)?)
            .map_err(Error::from)
            .map(|_| ())
    }
}
