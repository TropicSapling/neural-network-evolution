// $ cargo install wasm-bindgen
// $ cargo install wasm-bindgen-cli
// $ cargo build --target wasm32-unknown-unknown --release
// $ wasm-bindgen target/wasm32-unknown-unknown/debug/neural-network-evolution.wasm --target web --out-dir wasm

#[macro_use]
mod js;
mod structs;
mod game;
mod ai;

use wasm_bindgen::prelude::*;
use js::*;
use structs::*;
use game::update_game;
use ai::update_ai;

static mut AGENTS: Vec<Agent> = vec![];

fn main() {
	println!("Don't run it this way; compile to wasm!")
}

#[wasm_bindgen(start)]
pub unsafe fn start() {
	// Just for testing
	AGENTS.push(Agent {
		neurons : vec![],
		colour  : Colour {r: 67, g: 45, b: 123},
		pos     : Pos    {x: 100.0, y: 100.0},
		size    : 123,
		angle   : 0.0,

		moving  : true,
		turning : true
	});
}

#[wasm_bindgen]
pub unsafe fn run() {
	update_ai(&AGENTS);
	update_game(&mut AGENTS);
	draw_frame(&AGENTS);
}

fn draw_frame(agents: &Vec<Agent>) {
	console_log!("Redrawing frame.");
	draw_bg();
	for agent in agents {
		let Colour {r, g, b} = agent.colour;
		let Pos    {x, y}    = agent.pos;

		draw_agent(r, g, b, x, y, agent.size);
	}
}
