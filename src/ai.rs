use std::f64::consts::PI;

use crate::{structs::*, game::GAME_SIZE};

const MAX_DIST: f64 = 259_200_000_000.0;

pub fn update_ai(agents: &mut Vec<Agent>) {
	for i in 0..agents.len() {
		if agents[i].body.size < 32.0 {
			continue // for performance reasons, small agents are just stationary food
		}

		let (near_size, near_inv_dist, near_angle) = get_nearest(agents, i);

		let agent = &mut agents[i];
		let body  = &mut agent.body;
		let input = agent.brain.input();

		// Input neurons always fire
		(input[0].excitation, input[0].act_threshold) = (0.0, 0.0);
		(input[1].excitation, input[1].act_threshold) = (0.0, 0.0);
		(input[2].excitation, input[2].act_threshold) = (0.0, 0.0);

		// Relative size of nearest as first input
		for conn in &mut input[0].next_conn {
			conn.weight = if body.size > near_size*1.1 {
				1.0
			} else if near_size > body.size*1.1 {
				-1.0
			} else {0.0}
		}

		// Distance to nearest as second input
		for conn in &mut input[1].next_conn {
			conn.weight = near_inv_dist / MAX_DIST
		}

		// Angle towards nearest as third input
		for conn in &mut input[2].next_conn {
			conn.weight = near_angle / PI
		}

		// Input -> ... -> Output
		let output = agent.brain.update_neurons();

		// Movement output
		body.mov = 0.0;
		if output[0].excitation >= output[0].act_threshold {
			for conn in &output[0].next_conn {
				if conn.relu {
					body.mov += conn.weight * output[0].excitation
				} else {
					body.mov += conn.weight
				}
			}
		}

		// Rotation output
		body.rot = 0.0;
		if output[1].excitation >= output[1].act_threshold {
			for conn in &output[1].next_conn {
				if conn.relu {
					body.rot += conn.weight * output[1].excitation
				} else {
					body.rot += conn.weight
				}
			}
		}
	}
}

fn get_nearest(agents: &Vec<Agent>, i: usize) -> (f64, f64, f64) {
	let mut nearest = (0, 0.0, 0.0); // (ID, inv_dist, angle)
	let (pos,angle) = (agents[i].body.pos, agents[i].body.angle);

	// Calculate inverse distances and angles to borders
	let border_invdists_angles = [
		((GAME_SIZE - pos.x.abs()).powf(4.0), norm_angle(PI     - angle)),
		((GAME_SIZE - pos.y.abs()).powf(4.0), norm_angle(PI/2.0 - angle)),
		(pos.x.powf(4.0), norm_angle(-angle)),
		(pos.y.powf(4.0), norm_angle(-PI/2.0 - angle))
	];

	// Find the nearest agent
	for j in 0..agents.len() {
		if i == j || agents[j].body.size < 32.0 {continue}

		let inv_dist_to_j = inv_dist(pos, agents[j].body.pos);
		if inv_dist_to_j > nearest.1 {
			nearest = (j, inv_dist_to_j, angle_between(&agents, i, j))
		}
	}

	// Override nearest with border if it is closer
	for (dist, angle) in border_invdists_angles {
		if dist > nearest.1 {
			nearest = (usize::MAX, dist, angle)
		}
	}

	// Return with max size if border, otherwise with size of nearest agent
	if nearest.0 == usize::MAX {
		(GAME_SIZE, nearest.1, nearest.2)
	} else {
		(agents[nearest.0].body.size, nearest.1, nearest.2)
	}
}

fn norm_angle(angle: f64) -> f64 {
	angle.sin().atan2(angle.cos()) // trick to get angle between within [-PI, PI]
}

// Note: returns radians within [-PI, PI]
fn angle_between(agents: &Vec<Agent>, i: usize, j: usize) -> f64 {
	let pos  = agents[i].body.pos;
	let pos2 = agents[j].body.pos;
	let diff = agents[i].body.angle - (pos2.y - pos.y).atan2(pos2.x - pos.x);

	norm_angle(diff)
}

fn inv_dist(pos1: Pos, pos2: Pos) -> f64 {
	(GAME_SIZE - (pos1.x - pos2.x).abs()).powf(4.0) +
	(GAME_SIZE - (pos1.y - pos2.y).abs()).powf(4.0)
}
