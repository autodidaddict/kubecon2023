cargo build --release
wasm-tools component new --output ../deploy/wasmcloudfill.component.wasm --adapt ../wasi_snapshot_preview1.wasm ./target/wasm32-wasi/release/wasmcloud_fill.wasm
