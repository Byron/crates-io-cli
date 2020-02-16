#![deny(unsafe_code)]
/*!
Prodash is a dashboard for displaying the progress of concurrent application.

It consists of two parts

* a `Tree` to gather progress information and messages
* a terminal user interface which displays this information, along with optional free-form information provided by the application itself

Even though the `Tree` is not async, it's meant to be transparent and non-blocking performance wise, and benchmarks seem to indicate this
is indeed the case.

The **terminal user interface** seems to be the least transparent part, but can be configured to refresh less frequently.

# Examples

Please have a look at the [dashboard demo](https://github.com/Byron/crates-io-cli-rs/blob/master/prodash/examples/dashboard.rs).

*/
mod config;
pub mod tree;

pub use config::TreeOptions;
pub use tree::Root as Tree;

#[cfg(feature = "tui-renderer")]
pub mod tui;
