use crate::{Agent, helpers::*};

pub fn update_ai(agents: &mut Vec<Agent>) {
	for agent in agents {
		// TODO [...]

		let input  = &agent.brain.neurons_in;
		let output = &mut agent.brain.neurons_out;

		// Just for testing
		output[0].excitation = rand_range(0..1);
		output[1].excitation = rand_range(0..1);

		agent.body.moving  = output[0].excitation > output[0].act_threshold;
		agent.body.turning = output[1].excitation > output[1].act_threshold;
	}
}
