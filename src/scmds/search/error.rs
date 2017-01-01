use std::io;
use futures::sync::mpsc;
use rustc_serialize::json;
use tokio_curl::PerformError;
use super::structs::Command;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Curl(err: PerformError) {
            description("A curl request failed")
            from()
            cause(err)
        }
        Timeout(err: io::Error) {
            description("A timeout could not be created")
            cause(err)
        }
        Decode(err: json::DecoderError) {
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
