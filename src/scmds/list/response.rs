use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Crates {
    pub crates: Vec<Crate>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize)]
pub struct Crate {
    pub description: String,
    pub downloads: i64,
    pub max_version: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub total: u32,
}
