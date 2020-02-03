use serde_derive::{Deserialize, Serialize};

/// Represents a top-level crate and associated information
#[derive(Serialize, Deserialize)]
pub struct Crate {
    /// All versions published to crates.io, guaranteed to be sorted so that the most recent version is last.
    /// The format is as specified in Cargo.toml:version
    pub versions: Vec<String>,
}
