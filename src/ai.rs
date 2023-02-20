use crate::{Agent, js::*};

pub fn update_ai(agents: &mut Vec<Agent>) {
	for agent in agents {
		let body  = &mut agent.body;
		let input = &mut agent.brain.neurons_in;

		//let nearest = agent.get_nearest_of(&agents);
		let nearest = Agent::new(&mut vec![]); // dummy for now

		input[0].excitation = 1;
		input[1].excitation = 1;

		for conn in &mut input[0].next_conn {
			//conn.weight = agent.norm_dist_to(nearest_agent);
		}

		for conn in &mut input[1].next_conn {
			conn.weight = (body.size/nearest.body.size * 100.0) as isize;
			console_log!("size_diff={}", conn.weight); // debug
		}

		let output = agent.brain.update_neurons();

		body.moving  = output[0].excitation >= output[0].act_threshold;
		body.turning = output[1].excitation >= output[1].act_threshold;
	}
}
