use crate::{agent::*, input, output};

pub fn update_ai(agents: &mut Vec<Agent>) {
	for i in 0..agents.len() {
		if agents[i].body.size < 32.0 {
			continue // for performance reasons, small agents are just stationary food
		}

		let nearest = input::Nearest::to(agents, i);
		let agent   = &mut agents[i];

		// Input
		input::assign(agent.brain.input(), &agent.body, nearest);

		// Input -> ... -> Output
		let output = agent.brain.update_neurons();

		// Output
		output::assign(&mut agent.body.mov, &output[0]);
		output::assign(&mut agent.body.rot, &output[1]);
	}
}
