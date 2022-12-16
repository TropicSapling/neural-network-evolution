use crate::Agent;

const MOV_SPEED: f64 = 1.0;
const ROT_SPEED: f64 = 0.5;

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

fn handle_collisions(agents: &Vec<Agent>) {
	// TODO
}
