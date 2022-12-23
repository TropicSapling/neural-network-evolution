use crate::structs::*;

const GAME_SIZE: f64 = 600.0;

const MOV_SPEED: f64 = 2.0;
const ROT_SPEED: f64 = 0.02;

pub fn update_game(agents: &mut Vec<Agent>) {
	for agent in &mut *agents {
		mov(&mut agent.body);
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

fn handle_collisions(agents: &mut Vec<Agent>) {
	for agent in agents {
		let (pos, size) = (&mut agent.body.pos, agent.body.size);

		// Ensure no agent goes outside the game borders
		pos.x = pos.x.min(GAME_SIZE - size as f64).max(0.0);
		pos.y = pos.y.min(GAME_SIZE - size as f64).max(0.0);
	}

	// TODO
}
