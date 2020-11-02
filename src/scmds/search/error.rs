use super::structs::Command;
use crate::http_utils::RemoteCallError;
use futures::sync::mpsc;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ThreadPanic {
            description("The worker thread panicked")
        }
        ReactorInit(err: io::Error) {
            description("Could not initialize tokio event loop in worker thread")
            source(err)
        }
        Easy(err: RemoteCallError) {
            description("A remote call could not be performed")
            from()
            source(err)
        }
        Timeout(err: io::Error) {
            description("A timeout could not be created or failed while being invoked")
            source(err)
        }
        DecodeJson(err: serde_json::Error) {
            description("Json from the server could not be decoded")
            source(err)
            from()
        }
        SendCommand(err: mpsc::SendError<Command>) {
            description("A command could not be transmitted to the worker thread")
            source(err)
        }
        KeySequence(err: io::Error) {
            description("A keysequence on stdin could not be interpreted")
            source(err)
        }
        FirstIo(err: io::Error) {
            description("The first write to the output channel failed")
            source(err)
        }
        MissingRawTerminal (err: io::Error) {
            description("Standard output could not be put into raw mode")
            source(err)
            from()
        }
    }
}
