use crate::{Agent, Pos, js::*, helpers::*};

const MAX_DIST: f64 = 848.5281374238571;

pub fn update_ai(agents: &mut Vec<Agent>) {
	for i in 0..agents.len() {
		let nearest = get_nearest(agents, &agents[i]);

		let agent = &mut agents[i];
		let body  = &mut agent.body;
		let input = &mut agent.brain.neurons_in;

		// Input neurons always fire
		(input[0].excitation, input[0].act_threshold) = (1, 1);
		(input[1].excitation, input[1].act_threshold) = (1, 1);

		for conn in &mut input[0].next_conn {
			conn.weight = (nearest.1/MAX_DIST * 10.0) as isize
		}

		for conn in &mut input[1].next_conn {
			conn.weight = (body.size/nearest.0 * 5.0) as isize
		}

		// Debug
		if rand_range(0..256) == 0 {
			console_log!("dist={}", (nearest.1/MAX_DIST * 10.0) as isize);
			console_log!("size_diff={}", (body.size/nearest.0 * 5.0) as isize);
		}

		let output = agent.brain.update_neurons();

		body.moving  = output[0].excitation >= output[0].act_threshold;
		body.turning = output[1].excitation >= output[1].act_threshold;
	}
}

fn get_nearest(agents: &Vec<Agent>, agent: &Agent) -> (f64, f64) {
	let mut nearest = (0, MAX_DIST);

	for i in 0..agents.len() {
		let distance = dist(agent.body.pos, agents[i].body.pos);
		if distance != 0.0 && distance < nearest.1 {
			nearest = (i, distance)
		}
	}

	(agents[nearest.0].body.size, nearest.1)
}

fn dist(pos1: Pos, pos2: Pos) -> f64 {
	((pos1.x - pos2.x).powf(2.0) + (pos1.y - pos2.y).powf(2.0)).sqrt()
}
