// $ cargo install wasm-bindgen
// $ cargo install wasm-bindgen-cli
// $ cargo build --target wasm32-unknown-unknown
// $ wasm-bindgen target/wasm32-unknown-unknown/debug/neural-network-evolution.wasm --target web --out-dir wasm

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	#[wasm_bindgen(js_namespace = window)]
	fn draw_bg();

	#[wasm_bindgen(js_namespace = window)]
	fn draw_agent(r: usize, g: usize, b: usize, x: usize, y: usize, size: usize);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct Agent {
	neurons: Vec<Neuron>,
	colour: Colour,
	pos: Pos,
	size: usize
}

struct Colour {r: usize, g: usize, b: usize}
struct Pos    {x: usize, y: usize}

struct Neuron {
	excitation: isize,
	tick_drain: usize,

	act_threshold: usize,

	next_conn: Vec<ForwardConn>
}

struct ForwardConn {
	dest_index: usize,
	speed: usize,
	weight: isize
}

fn main() {
	println!("Don't run it this way; compile to wasm!")
}

#[wasm_bindgen]
pub fn run() {
	let agents = vec![];

	draw_frame(&agents);
}

fn draw_frame(agents: &Vec<Agent>) {
	draw_bg();
	for agent in agents {
		let Colour {r, g, b} = agent.colour;
		let Pos    {x, y}    = agent.pos;

		draw_agent(r, g, b, x, y, agent.size);
	}
}
