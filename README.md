![Rust](https://github.com/Byron/crates-io-cli/workflows/Rust/badge.svg)
[![crates.io version](https://img.shields.io/crates/v/crates-io-cli.svg)](https://crates.io/crates/crates-io-cli)

A command-line interface to interact with [crates.io](https://crates.io/)

### Features

* **search crates.io interactively and open in browser**
  [![asciicast](https://asciinema.org/a/40smybc7cmzeawrvttnh44es0.png)](https://asciinema.org/a/40smybc7cmzeawrvttnh44es0)
  
  [![asciicast](https://asciinema.org/a/99zkxo4gastj25qrp0zb0no4x.png)](https://asciinema.org/a/99zkxo4gastj25qrp0zb0no4x)
* **list recently changes crates**
  [![asciicast](https://asciinema.org/a/51qczytg4mh3aglhgczza0sot.png)](https://asciinema.org/a/51qczytg4mh3aglhgczza0sot)
* **output modes for humans and machines**
  [![asciicast](https://asciinema.org/a/0x0famma168b7xj663971gdsp.png)](https://asciinema.org/a/0x0famma168b7xj663971gdsp)

### Installation

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

### Usage

You can learn what it can do by using the `--help` flag:

```bash
crates --help
```
