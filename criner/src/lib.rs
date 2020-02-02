#[macro_use]
extern crate quick_error;

pub mod error;

use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub async fn run() -> Result<()> {
    unimplemented!()
}

pub fn run_blocking() -> Result<()> {
    async_std::task::block_on(run())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
