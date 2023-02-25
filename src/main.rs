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

// To my knowledge, this unfortunately has to be a mutable static to work with WASM
static mut AGENTS: Vec<Agent> = vec![];

////////////////////////////////

#[wasm_bindgen(start)]
pub unsafe fn start() {
	// Just for testing
	for _ in 0..rand_range(8..16) {
		AGENTS.push(Agent::new(&mut AGENTS));
	}

	console_log!("Starting version 0.1.9");
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

#[wasm_bindgen]
pub unsafe fn print_agent_at(x: f64, y: f64) {
	for agent in &AGENTS {
		let (pos, size) = (agent.body.pos, agent.body.size);

		if (pos.x..pos.x+size).contains(&x) && (pos.y..pos.y+size).contains(&y) {
			console_log!("Neural Network @ ({x}, {y}): {:#?}", agent.brain)
		}
	}
}

////////////////////////////////

fn main() {
	println!("Don't run it this way; compile to wasm!")
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
