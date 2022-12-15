// $ cargo install wasm-bindgen
// $ cargo install wasm-bindgen-cli
// $ cargo build --target wasm32-unknown-unknown
// $ wasm-bindgen target/wasm32-unknown-unknown/debug/neural-network-evolution.wasm --target web --out-dir wasm

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct Neuron {
	excitation: isize,
	tick_drain: usize,

	act_threshold: usize,

	next_conn: [ForwardConn]
}

struct ForwardConn {
	dest_index: usize,
	speed: usize,
	weight: isize
}

fn main() {
	println!("Don't run it this way, compile to wasm!")
}

#[wasm_bindgen(start)]
pub fn start() {
	console_log!("Hello, world!");
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
    console_log!("Hello, {name}!");
}

#[wasm_bindgen]
pub fn stop_neural_networks() {

}

#[wasm_bindgen]
pub fn stop_game() {

}
