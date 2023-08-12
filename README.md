# Wasmtime and libwasmtime, with crypto extensions

The currently stable version of the Wasmtime WebAssembly runtime,
with support for the
[WASI Cryptography APIs](https://github.com/WebAssembly/wasi-crypto).

## Installation

Install the Rust compiler and compile with:

```sh
cargo build --release
```

The resulting executable will be in the `target/release` directory.

The library (`libwasmtime.so`, `libwasmtime.dylib`, `libwasmtime.a`)
can be compiled with the following command:

```sh
cargo build --release -p wasmtime-c-api
```

The files can be found in `target/release` and
`crates/c-api/wasm-c-api/include`.

In this build, the function to enable WASI (`wasmtime_linker_define_wasi`)
will also enable the crypto extensions, so that no code change to
existing code depending on `libwasmtime` is necessary.

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
