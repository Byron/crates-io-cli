#[derive(RustcDecodable, Debug, Clone, Default)]
pub struct Crate {
    pub description: Option<String>,
    pub downloads: u32,
    pub max_version: String,
    pub name: String,
}
