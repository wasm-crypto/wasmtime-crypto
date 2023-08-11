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
