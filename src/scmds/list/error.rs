use crate::http_utils::RemoteCallError;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        DecodeJson(err: serde_json::Error) {
            description("Json from the server could not be decoded")
            source(err)
            from()
        }
        Easy(err: RemoteCallError) {
            description("A remote call could not be performed")
            from()
            source(err)
        }
        ReactorInit(err: io::Error) {
            description("Could not initialize tokio event loop in worker thread")
            source(err)
        }
    }
}
