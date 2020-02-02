//! If you are seeing this, than you see a very old way of doing things, prior
//! to 'async' being a keyword, and while all async was in its infancy.
//! Even though it would be nice to see how that code would look nowadays, I
//! would rather solve new problems than changing existing solutions, despite them
//! being pretty complex for what they do.

mod cmd;
mod error;

pub use self::{cmd::*, error::Error};
