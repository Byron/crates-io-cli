use std::fmt::{self, Display};
use std::default::Default;
use std::iter;
use std::str;
use std::cmp;

use rustc_serialize::json;
use termion::terminal_size;
use termion::clear;
use termion::cursor;

fn sanitize(input: &str) -> String {
    input.chars()
        .map(|c| if c == '\n' { ' ' } else { c })
        .collect()
}

#[derive(RustcDecodable)]
pub struct Dimension {
    pub width: u16,
    pub height: u16
}

impl Default for Dimension {
    fn default() -> Dimension {
        let (mw, mh) = terminal_size().unwrap_or((80, 20));
        Dimension {
            width: mw,
            height: mh
        }
    }
}

#[derive(RustcDecodable, Default)]
pub struct Meta {
    pub total: u32,
    pub dimension: Option<Dimension>,
}

#[derive(RustcDecodable, Debug, Clone, Default)]
pub struct Crate {
    pub description: String,
    pub downloads: u32,
    pub max_version: String,
    pub name: String,
}

impl Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.name.is_empty() {
            write!(f, "{clear}", clear = clear::AfterCursor)
        } else {
            write!(f,
                   "{name} | {desc:.80} | {downloads} | {version}",
                   name = self.name,
                   desc = sanitize(&self.description),
                   downloads = self.downloads,
                   version = self.max_version)
        }
    }
}

#[derive(RustcDecodable, Default)]
pub struct SearchResult {
    pub crates: Vec<Crate>,
    pub meta: Meta,
}

impl SearchResult {
    pub fn with_dimension() -> SearchResult {
        SearchResult {
            meta: Meta { dimension: Some(Dimension::default()), ..Default::default() },
            ..Default::default()
        }
    }
    pub fn from_data(buf: &[u8], dim: Dimension) -> Result<SearchResult, json::DecoderError> {
        str::from_utf8(buf)
            .map_err(|e| json::DecoderError::ApplicationError(format!("{}", e)))
            .and_then(json::decode)
            .map(|mut v: SearchResult| {
                v.meta.dimension = Some(dim);
                v
            })
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dim = self.meta.dimension.as_ref().expect("dimension to be set");
        for krate in self.crates
            .iter()
            .cloned()
            .chain(iter::repeat(Crate::default()))
            .take(dim.height as usize) {
            let krate = format!("{}", krate);
            write!(f,
                   "{clear}{:.max$}{down}{left}",
                   krate,
                   clear = clear::CurrentLine,
                   down = cursor::Down(1),
                   left = cursor::Left(cmp::max(krate.len(), dim.width as usize) as u16),
                   max = dim.width as usize)?;
        }
        Ok(())
    }
}
