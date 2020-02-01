use rustc_serialize::json;
use std::io;
use utils::RemoteCallError;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Decode(err: json::DecoderError) {
            description("Json from the server could not be decoded")
            cause(err)
            from()
        }
        Easy(err: RemoteCallError) {
            description("A remote call could not be performed")
            from()
            cause(err)
        }
        ReactorInit(err: io::Error) {
            description("Could not initialize tokio event loop in worker thread")
            cause(err)
        }
    }
}
