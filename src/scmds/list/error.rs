use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        ReactorInit(err: io::Error) {
            description("Could not initialize tokio event loop in worker thread")
            cause(err)
        }
    }
}
