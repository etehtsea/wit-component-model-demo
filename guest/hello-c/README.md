# Guest component (C)

### Build

- Get and setup [WASI SDK](https://github.com/webassembly/wasi-sdk#install).
- Install [wit-bindgen-cli](https://github.com/bytecodealliance/wit-bindgen#cli-installation)
- Install [wasm-tools](https://github.com/bytecodealliance/wasm-tools)

```sh
$ wit-bindgen c ../../wit
$ $CC demo.c demo_component_type.o hello-c.c -o hello-c.wasm -mexec-model=reactor
$ wasm-tools component new hello-c.wasm -o hello-c.component.wasm
```
