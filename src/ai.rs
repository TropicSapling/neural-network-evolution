use crate::Agent;

pub fn update_ai(agents: &mut Vec<Agent>) {
	for agent in agents {
		let output = agent.brain.update_neurons();

		agent.body.moving  = output[0].excitation >= output[0].act_threshold;
		agent.body.turning = output[1].excitation >= output[1].act_threshold;
	}
}
