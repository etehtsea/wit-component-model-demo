wasmtime::component::bindgen!({ async: true, path: "../../wit/demo.wit", world: "demo" });

const COMPONENT_RS: &str = "../../guest/hello-rs/target/hello-rs.component.wasm";
const COMPONENT_C: &str = "../../guest/hello-c/hello-c.component.wasm";

struct HostDemo {}

struct Context {
    demo: HostDemo,
    wasi: host::WasiCtx,
}

#[async_trait::async_trait]
impl imports::Imports for HostDemo {
    async fn print(&mut self, msg: String) -> anyhow::Result<()> {
        println!("{msg}");

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello from the host (Rust)!");

    let mut config = wasmtime::Config::new();
    config.wasm_component_model(true);
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    config.async_support(true);

    let engine = wasmtime::Engine::new(&config)?;
    let mut linker = wasmtime::component::Linker::new(&engine);

    let ctx = Context {
        demo: HostDemo {},
        wasi: wasi_cap_std_sync::WasiCtxBuilder::new()
            .inherit_stdio()
            .build(),
    };

    let mut store = wasmtime::Store::new(&engine, ctx);

    imports::add_to_linker(&mut linker, |ctx: &mut Context| &mut ctx.demo)?;
    host::add_to_linker(&mut linker, |ctx: &mut Context| &mut ctx.wasi)?;

    let guest_rs = wasmtime::component::Component::from_file(&engine, COMPONENT_RS)?;
    let guest_c = wasmtime::component::Component::from_file(&engine, COMPONENT_C)?;

    let (component_rs, _instance) = Demo::instantiate_async(&mut store, &guest_rs, &linker).await?;
    let (component_c, _instance) = Demo::instantiate_async(&mut store, &guest_c, &linker).await?;

    component_rs.exports.call_run(&mut store).await?;
    component_c.exports.call_run(&mut store).await?;

    Ok(())
}
