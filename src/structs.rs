#[derive(Debug, Clone, Default)]
pub struct Meta {
    pub total: u32,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Clone, Default)]
pub struct Crate {
    pub description: Option<String>,
    pub downloads: u32,
    pub max_version: String,
    pub name: String,
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    pub enum OutputKind {
        human,
        json
    }
}
