// $ cargo install wasm-bindgen
// $ cargo install wasm-bindgen-cli
// $ compile

#[macro_use]
mod js;
mod structs;
mod helpers;
mod game;
mod ai;

use wasm_bindgen::prelude::*;

use {helpers::*, js::*, structs::*};
use game::update_game;
use ai::update_ai;

static mut AGENTS: Vec<Agent> = vec![];

fn main() {
	println!("Don't run it this way; compile to wasm!")
}

#[wasm_bindgen(start)]
pub unsafe fn start() {
	// Just for testing
	for _ in 0..rand_range(8..16) {
		AGENTS.push(Agent::new(&mut AGENTS));
	}

	console_log!("Starting version 0.0.68");
}

#[wasm_bindgen]
pub unsafe fn run(inverse_spawn_rate: usize) {
	// Randomly spawn new agents
	if rand_range(0..inverse_spawn_rate) == 0 {
		AGENTS.push(Agent::new(&mut AGENTS));
	}

	update_ai(&mut AGENTS);
	update_game(&mut AGENTS);
	draw_frame(&AGENTS);
}

fn draw_frame(agents: &Vec<Agent>) {
	console_log!("Redrawing frame.");
	draw_bg();
	for agent in agents {
		let Colour {r, g, b} = agent.body.colour;
		let Pos    {x, y}    = agent.body.pos;

		draw_agent(r, g, b, x, y, agent.body.size);
	}
}
