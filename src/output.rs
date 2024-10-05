use crate::agent::Neuron;

pub fn assign(out: &mut f64, neuron: &Neuron) {
	*out = 0.0;
	if neuron.excitation >= neuron.act_threshold {
		for conn in &neuron.next_conn {
			if conn.relu {
				*out += conn.weight * neuron.excitation
			} else {
				*out += conn.weight
			}
		}
	}
}
