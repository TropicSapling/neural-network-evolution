use crate::Agent;

const GAME_SIZE: f64 = 600.0;

const MOV_SPEED: f64 = 1.0;
const ROT_SPEED: f64 = 0.1;

pub fn update_game(agents: &mut Vec<Agent>) {
	for mut agent in &mut *agents {
		mov(&mut agent);
	}

	handle_collisions(agents);
}

fn mov(agent: &mut Agent) {
	if agent.turning {
		agent.angle += ROT_SPEED;
	}

	if agent.moving {
		agent.pos.x += MOV_SPEED * agent.angle.cos();
		agent.pos.y += MOV_SPEED * agent.angle.sin();
	}
}

fn handle_collisions(agents: &mut Vec<Agent>) {
	for agent in agents {
		// Ensure no agent goes outside the game borders
		agent.pos.x = agent.pos.x.min(GAME_SIZE - agent.size as f64).max(0.0);
		agent.pos.y = agent.pos.y.min(GAME_SIZE - agent.size as f64).max(0.0);
	}

	// TODO
}
