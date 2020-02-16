#![deny(unsafe_code)]
/*!
Prodash is a dashboard for displaying the progress of concurrent application.

It consists of two parts

* a `Tree` to gather progress information and messages
* a terminal user interface which displays this information, along with optional free-form information provided by the application itself

Even though the `Tree` is not async, it's meant to be transparent and non-blocking performance wise, and benchmarks seem to indicate this
is indeed the case.

The **terminal user interface** seems to be the least transparent part, but can be configured to refresh less frequently.

*/
mod tree;

mod config;

pub use config::*;
pub use tree::*;

#[cfg(feature = "tui-renderer")]
pub mod tui;
