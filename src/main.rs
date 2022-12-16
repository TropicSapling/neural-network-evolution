// $ cargo install wasm-bindgen
// $ cargo install wasm-bindgen-cli
// $ cargo build --target wasm32-unknown-unknown
// $ wasm-bindgen target/wasm32-unknown-unknown/debug/neural-network-evolution.wasm --target web --out-dir wasm

#[macro_use]
mod js;
mod structs;

use wasm_bindgen::prelude::*;
use js::*;
use structs::*;

static mut AGENTS: Vec<Agent> = vec![];

fn main() {
	println!("Don't run it this way; compile to wasm!")
}

#[wasm_bindgen(start)]
pub unsafe fn init() {
	// Just for testing
	AGENTS.push(Agent {
		neurons : vec![],
		colour  : Colour {r: 67, g: 45, b: 123},
		pos     : Pos    {x: 123, y: 456},
		size    : 123
	});
}

#[wasm_bindgen]
pub unsafe fn run() {
	draw_frame(&AGENTS);
}

fn draw_frame(agents: &Vec<Agent>) {
	console_log!("Drawing frame.");
	draw_bg();
	for agent in agents {
		let Colour {r, g, b} = agent.colour;
		let Pos    {x, y}    = agent.pos;

		draw_agent(r, g, b, x, y, agent.size);
	}
}
