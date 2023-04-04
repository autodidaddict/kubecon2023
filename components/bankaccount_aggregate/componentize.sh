cargo build --release
wasm-tools component new --output ../deploy/bankaccountaggregate.component.wasm --adapt ../wasi_snapshot_preview1.wasm ./target/wasm32-wasi/release/bankaccount_aggregate.wasm
