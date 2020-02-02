use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Crates {
    pub crates: Vec<Crate>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Crate {
    pub description: Option<String>,
    pub downloads: i64,
    pub max_version: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub total: u32,
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum OutputKind {
        human,
        json
    }
}
