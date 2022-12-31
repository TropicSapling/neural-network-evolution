use crate::{js::*, structs::*};

const GAME_SIZE: f64 = 600.0;

const MOV_SPEED: f64 = 2.0;
const ROT_SPEED: f64 = 0.02;

pub fn update_game(agents: &mut Vec<Agent>) {
	for agent in &mut *agents {
		mov(&mut agent.body);
		shrink(&mut agent.body);
	}

	handle_collisions(agents);
}

fn mov(body: &mut Body) {
	if body.turning {
		body.angle += ROT_SPEED;
	}

	if body.moving {
		body.pos.x += MOV_SPEED * body.angle.cos();
		body.pos.y += MOV_SPEED * body.angle.sin();
	}
}

fn shrink(body: &mut Body) {
	let size0 = body.size;

	body.size  *= 0.999;
	body.pos.x += (size0 - body.size)/2.0;
	body.pos.y += (size0 - body.size)/2.0;
}

fn handle_collisions(agents: &mut Vec<Agent>) {
	for i in 0..agents.len() {
		if !agents[i].alive {continue} // skip dead agents

		let (pos, size) = (agents[i].body.pos, agents[i].body.size);

		// Check for collisions with other agents
		for j in 0..agents.len() {
			if i == j || !agents[j].alive {continue} // skip self & dead agents

			let (pos2, size2) = (agents[j].body.pos, agents[j].body.size);

			if closely_overlapping(pos, pos2, size, size2) {
				if size > size2*1.1 {
					// #i larger => eats #j
					eat(&mut agents[i].body, size, size2);
					agents[j].alive = false;
					console_log!("Agent#{i} ate Agent#{j}."); // debug
				} else if size2 > size*1.1 {
					// #j larger => eats #i
					eat(&mut agents[j].body, size2, size);
					agents[i].alive = false;
					console_log!("Agent#{j} ate Agent#{i}."); // debug
				}
			}
		}

		// Ensure no agent goes outside the game borders
		agents[i].body.pos.x = pos.x.min(GAME_SIZE - size as f64).max(0.0);
		agents[i].body.pos.y = pos.y.min(GAME_SIZE - size as f64).max(0.0);
	}

	// Remove dead agents
	agents.retain(|agent| agent.alive && agent.body.size > 32.0);

	// Sort agents by size so that larger ones are drawn on top of smaller ones
	agents.sort_unstable_by(|a, b| a.body.size.partial_cmp(&b.body.size).unwrap())
}

fn closely_overlapping(pos: Pos, pos2: Pos, size: f64, size2: f64) -> bool {
	// Calculates the overlapping area and returns true if >90% overlapping area

	let overlap_x1 = pos.x.max(pos2.x);
	let overlap_x2 = (pos.x+size).min(pos2.x+size2);

	let overlap_y1 = pos.y.max(pos2.y);
	let overlap_y2 = (pos.y+size).min(pos2.y+size2);

	let overlap_width  = overlap_x2 - overlap_x1;
	let overlap_height = overlap_y2 - overlap_y1;

	overlap_width                > 0.0                           &&
	overlap_height               > 0.0                           &&
	overlap_width*overlap_height > 0.9*size.min(size2).powf(2.0)
}

fn eat(eater: &mut Body, size_l: f64, size_s: f64) {
	let new_size_l = (size_l.powf(2.0) + size_s.powf(2.0)).sqrt();

	eater.pos.x -= (new_size_l - size_l)/2.0;
	eater.pos.y -= (new_size_l - size_l)/2.0;

	eater.size = new_size_l;
}
