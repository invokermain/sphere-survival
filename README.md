# Man VS Ball

An (in progress) attempt at making a simple 3D game in Rust using the [Fyrox library](https://fyrox.rs/). The goal is to be able to play it in the browser by compling to WASM.

## WASM commands

- `wasm-pack build --release --target web --out-dir ../web/dist/pkg --out-name wasm web`
- `basic-http-server web/dist`
