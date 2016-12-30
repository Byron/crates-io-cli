use utils::Dimension;

use std::fmt::{self, Display};
use std::default::Default;
use std::iter;
use std::str;
use std::cmp;

use rustc_serialize::json;
use termion::clear;
use termion::cursor;

const CRATE_ROW_OVERHEAD: u16 = 3 * 3;

fn sanitize(input: &str) -> String {
    input.chars()
        .map(|c| if c == '\n' { ' ' } else { c })
        .collect()
}

#[derive(RustcDecodable, Default)]
pub struct Meta {
    pub total: u32,
    pub dimension: Option<Dimension>,
}

#[derive(RustcDecodable, Debug, Clone, Default)]
pub struct Crate {
    pub description: Option<String>,
    pub downloads: u32,
    pub max_version: String,
    pub name: String,
}

struct CrateRow<'a>(&'a Crate, &'a (usize, usize, usize, usize));

impl<'a> Display for CrateRow<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let krate = &self.0;
        let &(nw, dw, vw, dlw) = self.1;
        if self.0.name.is_empty() {
            write!(f, "{clear}", clear = clear::AfterCursor)
        } else {
            write!(f,
                   "{name:nw$.nw$} | {desc:dw$.dw$} | {downloads:dlw$.dlw$} | {version:vw$.vw$}",
                   name = krate.name,
                   desc =
                       krate.description.as_ref().map(|d| sanitize(d)).unwrap_or_else(String::new),
                   downloads = krate.downloads,
                   version = krate.max_version,
                   nw = nw,
                   dw = dw,
                   dlw = dlw,
                   vw = vw)
        }
    }
}

fn desired_string_widths(items: &[Crate], max_width: u16) -> (usize, usize, usize, usize) {
    let (nw, dw, vw, dlw) = items.iter()
        .fold((0, 0, 0, 0), |(mut nw, mut dw, mut vw, mut dlw), c| {
            if c.name.len() > nw {
                nw = c.name.len();
            }
            let dlen =
                c.description.as_ref().map(|s| sanitize(&s)).unwrap_or_else(String::new).len();
            if dlen > dw {
                dw = dlen;
            }
            if c.max_version.len() > vw {
                vw = c.max_version.len();
            }
            let dllen = f64::log10(c.downloads as f64) as usize + 1;
            if dllen > dlw {
                dlw = dllen;
            }
            (nw, dw, vw, dlw)
        });
    let w = {
        let mut prio_widths = [dw, vw, dlw, nw];
        let max_width = max_width as usize;
        for (i, &w) in prio_widths.clone().iter().enumerate() {
            let total_width: usize = prio_widths[i..].iter().sum();
            prio_widths[i] = if total_width > max_width {
                w.saturating_sub(total_width - max_width)
            } else {
                w
            };
        }
        prio_widths
    };
    (w[3], w[0], w[1], w[2])
}

#[derive(RustcDecodable, Default)]
pub struct SearchResult {
    pub crates: Vec<Crate>,
    pub meta: Meta,
}

impl SearchResult {
    pub fn with_dimension(dim: Dimension) -> SearchResult {
        SearchResult {
            meta: Meta { dimension: Some(dim), ..Default::default() },
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
        let max_width = desired_string_widths(&self.crates, dim.width - CRATE_ROW_OVERHEAD);
        for krate in self.crates
            .iter()
            .cloned()
            .chain(iter::repeat(Crate::default()))
            .take(dim.height as usize) {
            let krate = format!("{}", CrateRow(&krate, &max_width));
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
