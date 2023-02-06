# Guest component (Rust)

### Build

```sh
$ rustup target add wasm32-wasi
$ curl -LO https://github.com/bytecodealliance/preview2-prototyping/releases/download/latest/wasi_snapshot_preview1.wasm
$ cargo build --target wasm32-wasi --release
$ cargo install wasm-tools
$ wasm-tools component new ./target/wasm32-wasi/release/hello_rs.wasm \
    -o ./target/hello-rs.component.wasm --adapt ./wasi_snapshot_preview1.wasm
```
