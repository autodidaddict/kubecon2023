cargo build --release
wasm-tools component new --output ../deploy/concordancefill.component.wasm --adapt ../wasi_snapshot_preview1.wasm ./target/wasm32-wasi/release/concordance_fill.wasm
