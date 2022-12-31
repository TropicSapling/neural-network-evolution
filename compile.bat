cargo build --target wasm32-unknown-unknown --release && wasm-bindgen target/wasm32-unknown-unknown/release/neural-network-evolution.wasm --target web --out-dir wasm && echo: && echo OK
