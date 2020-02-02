use super::structs::Command;
use crate::utils::RemoteCallError;
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
            cause(err)
        }
        Easy(err: RemoteCallError) {
            description("A remote call could not be performed")
            from()
            cause(err)
        }
        Timeout(err: io::Error) {
            description("A timeout could not be created or failed while being invoked")
            cause(err)
        }
        DecodeJson(err: serde_json::Error) {
            description("Json from the server could not be decoded")
            cause(err)
            from()
        }
        SendCommand(err: mpsc::SendError<Command>) {
            description("A command could not be transmitted to the worker thread")
            cause(err)
        }
        KeySequence(err: io::Error) {
            description("A keysequence on stdin could not be interpreted")
            cause(err)
        }
        FirstIo(err: io::Error) {
            description("The first write to the output channel failed")
            cause(err)
        }
        MissingRawTerminal (err: io::Error) {
            description("Standard output could not be put into raw mode")
            cause(err)
            from()
        }
    }
}
