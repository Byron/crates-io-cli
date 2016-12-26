[![Build Status](https://travis-ci.org/Byron/crates-io-cli-rs.svg?branch=master)](https://travis-ci.org/Byron/crates-io-cli-rs)

A command-line interface to interact with [crates.io](https://crates.io/)

# Features

* **list changes**
 * See what changed on crates.io.
* **multiple output modes**
 * Output for either *humans* or *machines* as *json*.

# Installation

If you have Rust on your system:
```bash
cargo install crates-io-cli
```

... and if you have to install from scratch, the following will install Rust and
the CLI at once:
```bash
{ command -v rustup 2>&1 >/dev/null || curl https://sh.rustup.rs -sSf | sh } && cargo install crates-io-cli
```

To install Rust on windows, you can follow the instrutions on [rustup.rs](https://rustup.rs).

# Usage

You can learn what it can do by using the `--help` flag:

```bash
crate --help
```
