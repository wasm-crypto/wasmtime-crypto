use wasi_crypto::wasmtime_interfaces::WasiCryptoCtx;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

struct Extensions {
    wasi: wasmtime_wasi::WasiCtx,
    wasi_crypto: WasiCryptoCtx,
}

fn add_wasi_crypto_to_linker<T>(
    linker: &mut Linker<T>,
    get_cx: impl Fn(&mut T) -> &mut WasiCryptoCtx + Send + Sync + Copy + 'static,
) -> anyhow::Result<()> {
    use wasi_crypto::wasmtime_interfaces::wasi_modules as w;

    w::wasi_ephemeral_crypto_common::add_to_linker(linker, get_cx)?;
    w::wasi_ephemeral_crypto_asymmetric_common::add_to_linker(linker, get_cx)?;
    w::wasi_ephemeral_crypto_signatures::add_to_linker(linker, get_cx)?;
    w::wasi_ephemeral_crypto_symmetric::add_to_linker(linker, get_cx)?;

    Ok(())
}

fn main() -> wasmtime::Result<()> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "test.wasm")?;
    let mut linker: Linker<Extensions> = Linker::new(&engine);

    let extensions = Extensions {
        wasi: WasiCtxBuilder::new().inherit_stdio().build(),
        wasi_crypto: WasiCryptoCtx::new(),
    };

    wasmtime_wasi::add_to_linker(&mut linker, |host| &mut host.wasi)?;
    add_wasi_crypto_to_linker(&mut linker, |host| &mut host.wasi_crypto)?;

    let mut store: Store<Extensions> = Store::new(&engine, extensions);
    let instance = linker.instantiate(&mut store, &module)?;

    let main = instance.get_typed_func::<(), ()>(&mut store, "_start")?;
    _ = main.call(&mut store, ());

    Ok(())
}
