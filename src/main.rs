// $ cargo install wasm-bindgen
// $ cargo install wasm-bindgen-cli
// $ cargo build --target wasm32-unknown-unknown --release
// $ wasm-bindgen target/wasm32-unknown-unknown/release/neural-network-evolution.wasm --target web --out-dir wasm

#[macro_use]
mod js;
mod structs;
mod game;
mod ai;

use std::{ops::Range, cmp::PartialOrd};
use rand::{Rng, distributions::uniform::SampleUniform};
use wasm_bindgen::prelude::*;

use js::*;
use structs::*;
use game::update_game;
use ai::update_ai;

static mut AGENTS: Vec<Agent> = vec![];

fn main() {
	println!("Don't run it this way; compile to wasm!")
}

fn rand_range<T: SampleUniform + PartialOrd>(range: Range<T>) -> T {
	rand::thread_rng().gen_range(range)
}

fn rand_col() -> Colour {
	Colour {r: rand_range(0..256), g: rand_range(0..256), b: rand_range(0..256)}
}

fn rand_pos() -> Pos {
	Pos {x: rand_range(0.0..450.0), y: rand_range(0.0..450.0)}
}

#[wasm_bindgen(start)]
pub unsafe fn start() {
	// Just for testing
	AGENTS.push(Agent {
		neurons : vec![],
		colour  : rand_col(),
		pos     : rand_pos(),
		size    : rand_range(64..128),
		angle   : 0.0,

		moving  : true,
		turning : true
	});

	console_log!("Spawned {:#?}.", AGENTS[0]);
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
