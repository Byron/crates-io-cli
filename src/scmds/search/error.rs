use crate::http_utils::RemoteCallError;
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The worker thread panicked")]
    ThreadPanic,
    #[error("A remote call could not be performed")]
    Easy(#[from] RemoteCallError),
    #[error("Json from the server could not be decoded")]
    DecodeJson(#[from] serde_json::Error),
    #[error("A command could not be transmitted to the worker thread")]
    SendCommand,
    #[error("A keysequence on stdin could not be interpreted")]
    KeySequence(#[source] io::Error),
    #[error("The first write to the output channel failed")]
    FirstIo(#[source] io::Error),
    #[error("Standard output could not be put into raw mode")]
    MissingRawTerminal(#[from] io::Error),
}
