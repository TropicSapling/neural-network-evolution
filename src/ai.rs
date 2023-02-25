use crate::structs::*;

const MAX_DIST: f64 = 259_200_000_000.0;
const MAX_DIFF: f64 = 150.0;

pub fn update_ai(agents: &mut Vec<Agent>) {
	for i in 0..agents.len() {
		if agents[i].body.size < 32.0 {
			continue // for performance reasons, small agents are just stationary food
		}

		let nearest = get_nearest(agents, i);

		let agent = &mut agents[i];
		let body  = &mut agent.body;
		let input = &mut agent.brain.neurons_in;

		// Input neurons always fire
		(input[0].excitation, input[0].act_threshold) = (0.0, 0.0);
		(input[1].excitation, input[1].act_threshold) = (0.0, 0.0);

		// Distance to nearest as first input
		for conn in &mut input[0].next_conn {
			conn.weight = nearest.0/MAX_DIST
		}

		// Relative size of nearest as second input
		for conn in &mut input[1].next_conn {
			conn.weight = body.size/nearest.1/MAX_DIFF // maybe should tweak?
		}

		// Input -> ... -> Output
		let output = agent.brain.update_neurons();

		// Movement output
		body.mov = 0.0;
		if output[0].excitation >= output[0].act_threshold {
			for conn in &output[0].next_conn {
				body.mov += conn.weight;
			}
		}

		// Rotation output
		body.rot = 0.0;
		if output[1].excitation >= output[1].act_threshold {
			for conn in &output[1].next_conn {
				body.rot += conn.weight;
			}
		}

		// Movement & rotation costs energy
		shrink_by(body, 0.9995_f64.powf(body.mov.abs() + body.rot.abs()))
	}
}

fn get_nearest(agents: &Vec<Agent>, i: usize) -> (f64, f64) {
	let mut nearest = (0.0, 0);

	for j in 0..agents.len() {
		if i == j {continue}

		let inv_distance = inv_dist(agents[i].body.pos, agents[j].body.pos);
		if inv_distance > nearest.0 {
			nearest = (inv_distance, j)
		}
	}

	(nearest.0, agents[nearest.1].body.size)
}

fn inv_dist(pos1: Pos, pos2: Pos) -> f64 {
	(600.0 - (pos1.x - pos2.x).abs()).powf(4.0) +
	(600.0 - (pos1.y - pos2.y).abs()).powf(4.0)
}

fn shrink_by(body: &mut Body, factor: f64) {
	let size0 = body.size;

	body.size  *= factor;
	body.pos.x += (size0 - body.size)/2.0;
	body.pos.y += (size0 - body.size)/2.0;
}
