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

#[derive(RustcDecodable, Default)]
pub struct Meta {
    pub total: u32,
    pub page_size: Option<usize>,
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
    pub fn with_page_size(page_size: usize) -> SearchResult {
        SearchResult {
            meta: Meta { page_size: Some(page_size), ..Default::default() },
            ..Default::default()
        }
    }
    pub fn from_data(buf: &[u8], page_size: usize) -> Result<SearchResult, json::DecoderError> {
        str::from_utf8(buf)
            .map_err(|e| json::DecoderError::ApplicationError(format!("{}", e)))
            .and_then(json::decode)
            .map(|mut v: SearchResult| {
                v.meta.page_size = Some(page_size);
                v
            })
    }
}

impl Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (mw, _mh) = terminal_size().map(|(w, h)| (w as usize, h as usize)).unwrap_or((80, 20));
        for krate in self.crates
            .iter()
            .cloned()
            .chain(iter::repeat(Crate::default()))
            .take(self.meta.page_size.as_ref().cloned().unwrap()) {
            let krate = format!("{}", krate);
            write!(f,
                   "{clear}{:.max$}{down}{left}",
                   krate,
                   clear = clear::CurrentLine,
                   down = cursor::Down(1),
                   left = cursor::Left(cmp::max(krate.len(), mw as usize) as u16),
                   max = mw)?;
        }
        Ok(())
    }
}
