use super::structs::Command;
use crate::http_utils::RemoteCallError;
use futures::sync::mpsc;
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The worker thread panicked")]
    ThreadPanic,
    #[error("Could not initialize tokio event loop in worker thread")]
    ReactorInit(#[source] io::Error),
    #[error("A remote call could not be performed")]
    Easy(#[from] RemoteCallError),
    #[error("A timeout could not be created or failed while being invoked")]
    Timeout(#[source] io::Error),
    #[error("Json from the server could not be decoded")]
    DecodeJson(#[from] serde_json::Error),
    #[error("A command could not be transmitted to the worker thread")]
    SendCommand(#[from] mpsc::SendError<Command>),
    #[error("A keysequence on stdin could not be interpreted")]
    KeySequence(#[source] io::Error),
    #[error("The first write to the output channel failed")]
    FirstIo(#[source] io::Error),
    #[error("Standard output could not be put into raw mode")]
    MissingRawTerminal(#[from] io::Error),
}
