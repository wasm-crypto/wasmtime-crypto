# Wasmtime, with crypto extensions

The currently stable version of the Wasmtime WebAssembly runtime,
with support for the
[WASI Cryptography APIs](https://github.com/WebAssembly/wasi-crypto).

## Installation

Install the Rust compiler and compile with:

```sh
cargo build --release
```

The resulting executable will be in the `target/release` directory.

## Usage

```sh
wasmtime run --wasi-modules experimental-wasi-crypto module.wasm
```

## WASI-Crypto when wasmtime is used as a Rust dependency

In Rust applications, `wasi-crypto` can be used directly with
the stock `wasmtime` crate downloaded from `crates.io`.

See the [`wasmtime-crate-usage-example`](wasmtime-crate-usage-example)
directory for an example that runs a WebAssembly module with WASI
and WASI-Crypto.
