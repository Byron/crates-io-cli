#[derive(RustcDecodable, Debug, Clone, Default)]
pub struct Meta {
    pub total: u32,
}

#[derive(RustcDecodable, Debug, Clone, Default)]
pub struct Crate {
    pub description: Option<String>,
    pub downloads: u32,
    pub max_version: String,
    pub name: String,
}
