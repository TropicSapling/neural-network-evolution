use std::f64::consts::PI;
use crate::{agent::*, game::GAME_SIZE};

const MAX_DIST: f64 = 259_200_000_000.0;

////////////////////////////////

pub fn assign(input: &mut [Neuron; 4], body: &Body, nearest: Nearest) {
	// Relative size of nearest as first input
	input[0].excitation = if body.size > nearest.size*1.1 {
		1.0
	} else if nearest.size > body.size*1.1 {
		-1.0
	} else {0.0};

	// Distance to nearest as second input
	input[1].excitation = nearest.inv_dist / MAX_DIST;

	// Angle towards nearest as third input
	input[2].excitation = nearest.angle / PI;

	// Touching edge or not as fourth input
	input[3].excitation = touching_edge(body).into()
}

fn touching_edge(body: &Body) -> bool {
	body.pos.x == 0.0                   ||
	body.pos.y == 0.0                   ||
	body.pos.x == GAME_SIZE - body.size ||
	body.pos.y == GAME_SIZE - body.size
}

////////////////////////////////

pub struct Nearest {
	size     : f64,
	inv_dist : f64,
	angle    : f64
}

impl Nearest {
	pub fn to(agents: &Vec<Agent>, i: usize) -> Self {
		let mut nearest = Nearest {
			size     : 0.0,
			inv_dist : 0.0,
			angle    : 0.0
		};

		// Find the nearest agent
		for j in 0..agents.len() {
			if i == j || agents[j].body.size < 32.0 {continue}

			let inv_dist_to_j = Self::inv_dist(agents[i].body.pos, agents[j].body.pos);
			if inv_dist_to_j > nearest.inv_dist {
				nearest = Nearest {
					size     : agents[j].body.size,
					inv_dist : inv_dist_to_j,
					angle    : Self::angle_between(&agents[i].body, &agents[j].body)
				}
			}
		}

		nearest
	}

	fn norm_angle(angle: f64) -> f64 {
		angle.sin().atan2(angle.cos()) // trick to get angle within [-PI, PI]
	}

	// Note: returns radians within [-PI, PI]
	fn angle_between(b1: &Body, b2: &Body) -> f64 {
		Self::norm_angle(b1.angle - (b2.pos.y - b1.pos.y).atan2(b2.pos.x - b1.pos.x))
	}

	fn inv_dist(pos1: Pos, pos2: Pos) -> f64 {
		(GAME_SIZE - (pos1.x - pos2.x).abs()).powf(4.0) +
		(GAME_SIZE - (pos1.y - pos2.y).abs()).powf(4.0)
	}
}
