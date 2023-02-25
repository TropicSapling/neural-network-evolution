use std::fmt;

use crate::helpers::*;

const OUTS: usize = 2;

#[derive(Debug)]
pub struct Agent {
	pub brain : Brain,
	pub body  : Body,
	pub alive : bool,

	inv_split_freq: usize
}


////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////


/// neurons_in  : [dist, size_diff] normalised to [0, 1]
/// neurons_out : [mov, rot]
#[derive(Clone, Debug)]
pub struct Brain {
	pub neurons_in     : [Neuron; 2],
	pub neurons_hidden : Vec<Neuron>,
	pub neurons_out    : [Neuron; 2],

	pub generation: usize // for debugging/display
}

#[derive(Clone)]
pub struct Neuron {
	pub excitation: f64,
	pub tick_drain: f64,

	pub act_threshold: f64,

	pub next_conn: Vec<ForwardConn>,

	reachable: bool
}

#[derive(Clone, Debug)]
pub struct ForwardConn {
	pub dest_index: usize,
	pub speed: usize, // currently unused
	pub weight: f64
}

// TODO - STDP (Spike-Timing-Dependent Plasticity):
// Strengthen/weaken connection weight if receiving neuron
// activates shortly after/before connection fired.


////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////


#[derive(Debug)]
pub struct Body {
	pub colour: Colour,

	pub pos   : Pos,
	pub size  : f64,
	pub angle : f64,

	pub mov: f64,
	pub rot: f64
}

#[derive(Clone, Debug)]
pub struct Colour {
	pub r: usize,
	pub g: usize,
	pub b: usize
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {pub x: f64, pub y: f64}


////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////


impl Agent {
	pub fn new(agents: &mut Vec<Agent>) -> Agent {
		for parent in agents {
			if parent.body.size > 80.0 && rand_range(0..parent.inv_split_freq) == 0 {
				// Spawn child agent

				let child_size = 0.7*parent.body.size;

				parent.body.remove(child_size); // shrink parent

				let freq   = parent.inv_split_freq;
				let colour = parent.body.colour.clone();
				let brain  = parent.brain.clone();

				return Agent::with(brain, colour, child_size, freq).mutate()
			}
		}

		// Spawn new random agent
		Agent::with(Brain {
			neurons_in     :     [Neuron::new(6), Neuron::new(6)],
			neurons_hidden : vec![Neuron::new(6), Neuron::new(6),
			                      Neuron::new(6), Neuron::new(6)],
			neurons_out    :     [Neuron::new(6), Neuron::new(6)],
			generation: 0
		}, Colour::new(), rand_range(8.0..80.0), 16)
	}

	fn with(brain: Brain, colour: Colour, size: f64, freq: usize) -> Agent {
		Agent {
			brain,

			body: Body {
				colour,
				pos: Pos::new(),
				size,
				angle: rand_range(0.0..360.0),

				mov: 0.0,
				rot: 0.0
			},

			alive: true,

			inv_split_freq: freq
		}
	}

	fn mutate(mut self) -> Self {
		let recv_neuron_count = self.brain.neurons_hidden.len() + OUTS;

		// Slightly mutate colours
		self.body.colour.r.add_bounded_max(rand_range(-16..16), 256);
		self.body.colour.g.add_bounded_max(rand_range(-16..16), 256);
		self.body.colour.b.add_bounded_max(rand_range(-16..16), 256);

		// Mutate inverse split frequency
		if rand_range(0..self.inv_split_freq) == 0 {
			if rand_range(0..=1) == 0 || self.inv_split_freq <= 1 {
				self.inv_split_freq *= 2
			} else {
				self.inv_split_freq /= 2
			}
		} else {
			self.inv_split_freq.add_bounded(rand_range(-1..=1))
		}

		let mut new_neuron_count = 0;

		// Mutate input neurons
		for neuron in &mut self.brain.neurons_in {
			new_neuron_count += neuron.mutate(recv_neuron_count)
		}

		// Mutate hidden neurons
		for neuron in &mut self.brain.neurons_hidden {
			new_neuron_count += neuron.mutate(recv_neuron_count)
		}

		// Mutate output neurons
		for neuron in &mut self.brain.neurons_out {
			new_neuron_count += neuron.mutate(recv_neuron_count)
		}

		// Add new hidden neurons
		for _ in 0..new_neuron_count {
			self.brain.neurons_hidden.push(Neuron::new(recv_neuron_count))
		}

		self.brain.generation += 1;

		self
	}
}


////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////


impl Brain {
	pub fn update_neurons(&mut self) -> &[Neuron; 2] {
		// Drain output neurons from previous excitation
		for i in 0..OUTS {
			self.neurons_out[i].drain()
		}

		for i in 0..self.neurons_in.len() {
			self.neurons_in[i].reachable = true; // input neurons always reachable
			self.update_neuron(i, true)
		}

		for i in 0..self.neurons_hidden.len() {
			if self.neurons_hidden[i].reachable {
				self.update_neuron(i, false);
				self.neurons_hidden[i].drain()
			}
		}

		&self.neurons_out
	}

	fn update_neuron(&mut self, i: usize, is_input: bool) {
		let neuron = match is_input {
			true => &self.neurons_in[i],
			_    => &self.neurons_hidden[i]
		};

		// If neuron activated...
		if neuron.excitation >= neuron.act_threshold {
			// ... prepare all connections for activation
			let mut activations = vec![];
			for conn in &neuron.next_conn {
				activations.push(conn.clone())
			}

			// ... and then activate the connections
			for conn in activations {
				let recv_neuron = if conn.dest_index < OUTS {
					&mut self.neurons_out[conn.dest_index]
				} else {
					&mut self.neurons_hidden[conn.dest_index - OUTS]
				};

				recv_neuron.excitation += conn.weight;
				recv_neuron.reachable   = true
			}
		}
	}
}

impl Neuron {
	fn new(recv_neuron_count: usize) -> Neuron {
		Neuron {
			excitation: 0.0,
			tick_drain: 1.0,

			act_threshold: 0.5,

			next_conn: vec![ForwardConn {
				dest_index: rand_range(0..recv_neuron_count),
				speed: 0,
				weight: [-1.0, 1.0][rand_range(0..=1)]
			}],

			reachable: false
		}
	}

	// 50/50 if mutation or not
	fn should_mutate_now() -> bool {rand_range(0..=1) == 1}

	fn mutate(&mut self, recv_neuron_count: usize) -> usize {
		let mut new_neuron_count = 0;
		let mut new_conn_count   = 0;

		// Mutate neuron properties
		if Neuron::should_mutate_now() {
			self.tick_drain += [-1.0, 1.0][rand_range(0..=1)]}
		if Neuron::should_mutate_now() {
			self.act_threshold += [-1.0, 1.0][rand_range(0..=1)]}

		// Mutate outgoing connections
		for conn in &mut self.next_conn {
			if Neuron::should_mutate_now() {
				let mut_this = rand_range(0..=1) == 0;
				let add_conn = rand_range(0..=1) == 0;

				if mut_this {
					conn.weight += [-1.0, 1.0][rand_range(0..=1)]
				} else if add_conn {
					new_conn_count += 1
				} else {
					new_neuron_count += 1
				}
			}
		}

		// Add new outgoing connections
		for _ in 0..new_conn_count {
			self.next_conn.push(ForwardConn {
				dest_index: rand_range(0..recv_neuron_count),
				speed: 0,
				weight: [-1.0, 1.0][rand_range(0..=1)]
			})
		}

		// Remove effectively dead connections
		self.next_conn.retain(|conn| (conn.weight*10.0).round() != 0.0);

		// Assume not reachable until proven otherwise
		self.reachable = false;

		new_neuron_count
	}

	fn drain(&mut self) {
		// Drain excitation towards a neutral state of 0
		if self.excitation > 0.0 {
			self.excitation -= self.tick_drain.abs();
			if self.excitation < 0.0 {
				self.excitation = 0.0
			}
		} else {
			self.excitation += self.tick_drain.abs();
			if self.excitation > 0.0 {
				self.excitation = 0.0
			}
		}
	}
}

impl fmt::Debug for Neuron {
	// Print neuron debug info in a concise way
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if !self.reachable {
			if self.next_conn.len() < 1 {
				write!(f, "Neuron {{UNREACHABLE & INACTIVE}}")
			} else {
				write!(f, "Neuron {{UNREACHABLE}}")
			}
		} else if self.next_conn.len() < 1 {
			write!(f, "Neuron {{INACTIVE}}")
		} else {
			let mut s = format!("Neuron {{ACT@{} | ", self.act_threshold);

			let mut conn_iter = self.next_conn.iter().peekable();
			while let Some(conn) = conn_iter.next() {
				s += &format!("(*{:.1})->#{}", conn.weight, conn.dest_index);
				if !conn_iter.peek().is_none() {
					s += ", "
				}
			}

			write!(f, "{s}}}")
		}
	}
}


////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////


impl Body {
	fn remove(&mut self, removal: f64) {
		let new_size = (self.size*self.size - removal*removal).sqrt();

		self.pos.x += (self.size - new_size)/2.0;
		self.pos.y += (self.size - new_size)/2.0;

		self.size = new_size;
	}
}

impl Colour {
	fn new() -> Colour {
		Colour {r: rand_range(0..256), g: rand_range(0..256), b: rand_range(0..256)}
	}
}

impl Pos {
	fn new() -> Pos {
		Pos {x: rand_range(0.0..450.0), y: rand_range(0.0..450.0)}
	}
}
