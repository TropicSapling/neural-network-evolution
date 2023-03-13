// $ cargo install wasm-bindgen
// $ cargo install wasm-bindgen-cli
// $ compile

#[macro_use]
mod js;
mod structs;
mod helpers;
mod game;
mod ai;

use std::sync::Mutex;

use wasm_bindgen::prelude::*;

use {helpers::*, js::*, structs::*};
use game::update_game;
use ai::update_ai;

// Need static mutex to work with WASM
static AGENTS: Mutex<Vec<Agent>> = Mutex::new(vec![]);

////////////////////////////////

#[wasm_bindgen(start)]
pub fn start() {
	console_log!("Starting version 0.2.5")
}

#[wasm_bindgen]
pub fn run(inverse_spawn_rate: usize) {
	let mut agents = AGENTS.lock().unwrap();

	// Randomly spawn new agents
	if rand_range(0..inverse_spawn_rate) == 0 {
		let agent = Agent::new(&agents);
		agents.push(agent)
	}

	if let Some(agent) = Agent::maybe_split(&mut agents) {
		agents.push(agent)
	}

	update_ai(&mut agents);
	update_game(&mut agents);
	draw_frame(&agents);
}

#[wasm_bindgen]
pub fn print_agent_at(x: f64, y: f64) {
	let agents = AGENTS.lock().unwrap();
	for agent in agents.iter() {
		let (pos, size) = (agent.body.pos, agent.body.size);

		if (pos.x..pos.x+size).contains(&x) && (pos.y..pos.y+size).contains(&y) {
			let network = format!("Neural Network @ ({x}, {y}): {:#?}", agent.brain);

			console_log!("{network}\n\nAGENTS ALIVE: {}", agents.len());
			draw_neural_network(network);
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

		draw_agent(r, g, b, x, y, agent.body.size)
	}
}
