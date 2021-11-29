use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;

use crate::task::Task;

pub struct WasmRuntime<Task> {
    store: Store<Task>,
    engine: Engine,
    linker: Linker<Task>,
}

impl WasmRuntime<Task> {
    fn default() -> Self {
        // Define the WASI functions globally on the `Config`.
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);

        // Create a WASI context and put it in a Store; all instances in the store
        // share this context. `WasiCtxBuilder` provides a number of ways to
        // configure what the target program will have access to.
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()?
            .build();

        // Create the store instance
        let mut store = Store::new(&engine, wasi);

        // Return the initialized runtime.
        WasmRuntime {
            store,
            engine,
            linker,
        }
    }

    fn load_wasm_module(&self, module_name: String) -> Result<()> {
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;

        // TODO: Retrieve from the OCI registry.
        // Instantiate our module with the imports we've created, and run it.
        let module = Module::from_file(&engine, "target/wasm32-wasi/debug/wasi.wasm")?;

        // Link the module by name to the store instance.
        linker.module(&mut store, module_name.as_str(), &module)?;
        linker
            .get_default(&mut store, "")?
            .typed::<(), (), _>(&store)?
            .call(&mut store, ())?;

        Ok(())
    }
}
