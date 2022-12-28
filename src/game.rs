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
	body.size *= 0.999;
}

fn handle_collisions(agents: &mut Vec<Agent>) {
	let mut eaten = vec![];

	for i in 0..agents.len() {
		let (pos, size) = (agents[i].body.pos, agents[i].body.size);

		// Ensure no agent goes outside the game borders
		agents[i].body.pos.x = pos.x.min(GAME_SIZE - size as f64).max(0.0);
		agents[i].body.pos.y = pos.y.min(GAME_SIZE - size as f64).max(0.0);

		// Check for collisions with other agents
		for j in 0..agents.len() {
			if i != j {
				let (pos2, size2) = (agents[j].body.pos, agents[j].body.size);

				// TODO: fix, something is not quite right!
				let diff1 = (pos.x      - pos2.x      , pos.y      - pos2.y      );
				let diff2 = (pos.x+size - pos2.x+size2, pos.y+size - pos2.y+size2);

				if diff1 < (-5.0, -5.0) && diff2 > (5.0, 5.0) {
					// #i larger => eats #j
					eat(&mut agents[i].body, size, size2);
					eaten.push(j);
					console_log!("Agent#{i} ate Agent#{j}."); // debug
				} else if diff1 > (5.0, 5.0) && diff2 < (-5.0, -5.0) {
					// #j larger => eats #i
					eat(&mut agents[j].body, size2, size);
					eaten.push(i);
					console_log!("Agent#{j} ate Agent#{i}."); // debug
				}
			}
		}
	}

	for i in eaten {
		agents.remove(i);
	}
}

fn eat(eater: &mut Body, size_l: f64, size_s: f64) {
	let new_size_l = (size_l.powf(2.0) + size_s.powf(2.0)).sqrt();

	eater.pos.x -= (new_size_l - size_l)/2.0;
	eater.pos.y -= (new_size_l - size_l)/2.0;

	eater.size = new_size_l;
}
