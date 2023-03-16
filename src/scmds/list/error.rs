use crate::http_utils::RemoteCallError;
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Json from the server could not be decoded")]
    DecodeJson(#[from] serde_json::Error),
    #[error("A remote call could not be performed")]
    Easy(#[from] RemoteCallError),
    #[error("Could not initialize tokio event loop in worker thread")]
    ReactorInit(#[from] io::Error),
}
