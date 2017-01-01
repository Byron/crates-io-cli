use std::io;
use futures::sync::mpsc;
use super::structs::Command;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
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
