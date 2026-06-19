use curl::easy::Easy;

use curl;
use std::{
    cmp,
    default::Default,
    error::Error,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

const MAX_ITEMS_PER_PAGE: u32 = 100;

pub type CallResult = (Arc<Mutex<Vec<u8>>>, Easy);
pub type CancelFlag = Arc<AtomicBool>;

#[cfg(feature = "search")]
pub fn new_cancel_flag() -> CancelFlag {
    Arc::new(AtomicBool::new(false))
}

fn remote_call(url: &str, cancel: Option<CancelFlag>) -> Result<CallResult, RemoteCallError> {
    let mut req = request_new();
    req.get(true)?;
    req.url(url)?;
    if let Some(cancel) = cancel {
        req.progress(true)?;
        req.progress_function(move |_, _, _, _| !cancel.load(Ordering::Relaxed))?;
    }
    let buf = Arc::new(Mutex::new(Vec::new()));
    let buf_handle = buf.clone();
    req.write_function(move |data| {
        buf_handle.lock().unwrap().extend_from_slice(data);
        Ok(data.len())
    })?;
    req.perform()?;
    Ok((buf, req))
}

fn request_new() -> Easy {
    let mut easy = Easy::new();
    easy.useragent("crates.io-cli (https://crates.io/crates/crates-io-cli)")
        .ok();
    easy.timeout(Duration::from_secs(15)).ok();
    easy
}

#[derive(Debug, thiserror::Error)]
pub enum RemoteCallError {
    #[error("Easy curl could not be configured")]
    Easy(#[from] curl::Error),
    #[error("An error occurred")]
    Any(#[from] Box<dyn Error + Send + 'static>),
}

#[derive(Default)]
pub struct CallMetaData {
    /// total amount of items as reported by crates.io in `meta.total`
    pub total: u32,
    /// amount of items seen in the current call
    pub items: u32,
}

pub fn paged_crates_io_remote_call<T, M, E, Err>(
    url: &str,
    max_items: Option<u32>,
    merge: M,
    extract: E,
    cancel: Option<CancelFlag>,
) -> Result<T, RemoteCallError>
where
    T: Default,
    Err: Error + Send + 'static,
    M: Fn(T, CallResult) -> Result<T, Err>,
    E: FnOnce(CallResult) -> Result<(CallMetaData, T), Err>,
{
    let max_items = max_items.unwrap_or(u32::MAX);
    let page_size = cmp::min(MAX_ITEMS_PER_PAGE, max_items);

    let url = url.to_owned();
    let (m, mut result) = extract(remote_call(
        &format!("{}&per_page={}", url, page_size),
        cancel.clone(),
    )?)
    .map_err(|e| RemoteCallError::Any(Box::new(e)))?;

    let remaining_items = cmp::min(
        m.total.saturating_sub(m.items),
        max_items.saturating_sub(m.items),
    );
    let remaining_pages = remaining_items.div_ceil(page_size);

    for page in 0..remaining_pages {
        let call = remote_call(
            &format!("{}&page={}&per_page={}", url, page + 2, page_size),
            cancel.clone(),
        )?;
        result = merge(result, call).map_err(|e| RemoteCallError::Any(Box::new(e)))?;
    }

    Ok(result)
}
