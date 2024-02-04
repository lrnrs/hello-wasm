# Hello wasm

### Setup
```shell
cargo install wasm-pac
```

### Packing
```shell
wasm-pack build --target web
```

### Serving
```shell
python3 -m http.server
```

### Purpose

The core idea is to be a sandbox to connect Rust to a simple website.
Currently, the goal is: generate SVG in Rust and expose it through wasm to javascript, and there, place it in a canvas