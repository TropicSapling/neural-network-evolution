use std::{fmt, f64::consts::PI};

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

/// neurons_in  : [dist, size_diff, angle_to_near] normalised to [-1, 1]
/// neurons_out : [mov, rot] normalised to [-1, 1]
#[derive(Clone)]
pub struct Brain {
	pub neurons_in     : [Neuron; 3],
	pub neurons_hidden : Vec<Neuron>,
	pub neurons_out    : [Neuron; 2],

	pub generation: usize // for debugging/display
}

#[derive(Clone)]
pub struct Neuron {
	pub excitation: f64,
	pub tick_drain: f64,

	pub act_threshold: f64,

	pub next_conn: Vec<OutwardConn>,

	reachable: bool
}

#[derive(Clone, Debug)]
pub struct OutwardConn {
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
	pub fn new() -> Agent {
		let mut new_agent = Agent::with(Brain {
			neurons_in     :     [Neuron::new(8), Neuron::new(8), Neuron::new(8)],
			neurons_hidden : vec![Neuron::new(8), Neuron::new(8), Neuron::new(8),
			                      Neuron::new(8), Neuron::new(8), Neuron::new(8)],
			neurons_out    :     [Neuron::new(8),                 Neuron::new(8)],
			generation: 0
		}, Colour::new(), rand_range(48.0..64.0), 256);

		for _ in 0..rand_range(0..16) {
			new_agent = new_agent.mutate()
		}

		new_agent.brain.generation = 0;

		new_agent
	}

	pub fn maybe_split(agents: &mut Vec<Agent>) -> Option<Agent> {
		// TODO: consider instead spawning children of all-time high scorers
		for parent in agents {
			if parent.body.size > 64.0 {
				let div        = 1.0 + (parent.body.size - 64.0)/16.0;
				let inv_chance = parent.inv_split_freq / (div as usize);

				// TODO: decide when to split based on a third neuron output instead?
				if rand_range(0..=inv_chance) == 0 {
					// Spawn child agent

					let child_size = 0.7*parent.body.size;

					parent.body.remove(child_size); // shrink parent

					let freq   = parent.inv_split_freq;
					let colour = parent.body.colour.clone();
					let brain  = parent.brain.clone();

					// Spawn identical copy of self in 1/3 of cases, otherwise mutate
					return if rand_range(0..3) == 0 {
						Some(Agent::with(brain, colour, child_size, freq))
					} else {
						Some(Agent::with(brain, colour, child_size, freq).mutate())
					}
				}
			}
		}

		None
	}

	fn with(brain: Brain, colour: Colour, size: f64, freq: usize) -> Agent {
		Agent {
			brain,

			body: Body {
				colour,
				pos: Pos::new(),
				size,
				angle: rand_range(-PI..PI),

				mov: 0.0,
				rot: 0.0
			},

			alive: true,

			inv_split_freq: freq
		}
	}

	fn mutate(mut self) -> Self {
		let mut recv_neurons = self.brain.neurons_hidden.len() + OUTS;

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

		let mut new_neurons = 0;
		let mut new_conns   = 0;

		// Mutate input neurons
		for neuron in &mut self.brain.neurons_in {
			neuron.mutate(&mut new_neurons, &mut new_conns, recv_neurons)
		}

		// Mutate hidden neurons
		for neuron in &mut self.brain.neurons_hidden {
			neuron.mutate(&mut new_neurons, &mut new_conns, recv_neurons)
		}

		// Mutate output neurons
		for neuron in &mut self.brain.neurons_out {
			neuron.mutate(&mut new_neurons, &mut new_conns, recv_neurons)
		}

		// Add new hidden neurons
		for _ in 0..new_neurons {
			self.brain.neurons_hidden.push(Neuron::new(recv_neurons));
			recv_neurons += 1
		}

		// Add new outgoing connections
		for _ in 0..new_conns {
			let inps = self.brain.neurons_in.len();
			let hids = self.brain.neurons_hidden.len();
			let rand = rand_range(0..inps+hids+OUTS);

			let neuron = if rand < inps {
				&mut self.brain.neurons_in[rand]
			} else if rand < inps+hids {
				&mut self.brain.neurons_hidden[rand-inps]
			} else {
				&mut self.brain.neurons_out[rand-inps-hids]
			};

			neuron.next_conn.push(OutwardConn::new(recv_neurons))
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

				// TODO: ReLU (+= weight*excitation)
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

			act_threshold: 0.0,

			next_conn: vec![OutwardConn::new(recv_neuron_count)],

			reachable: false
		}
	}

	// TODO: maybe have mutation rate part of neuron properties?

	// 20/80 if mutation or not
	fn should_mutate_now() -> bool {rand_range(0..5) == 0}
	// 50/50 if expansion or shrinking
	fn should_expand_now() -> bool {rand_range(0..2) == 0}

	fn mutate(&mut self,
		new_neuron_count  : &mut usize,
		new_conn_count    : &mut usize,
		recv_neuron_count :      usize
	) {
		// Mutate neuron properties
		if Neuron::should_mutate_now() {
			self.tick_drain += [-1.0, 1.0][rand_range(0..=1)]}
		if Neuron::should_mutate_now() {
			self.act_threshold += [-1.0, 1.0][rand_range(0..=1)]}

		// Mutate outgoing connections
		for conn in &mut self.next_conn {
			if Neuron::should_mutate_now() {
				if rand_range(0..=(conn.weight.abs() as usize)) == 0 {
					// Sometimes flip weight
					conn.weight = -conn.weight
				} else {
					if Neuron::should_expand_now() {
						// Sometimes expand weight or other stuff
						match rand_range(0..3) {
							0 => Neuron::expand_or_shrink(&mut conn.weight, 1.0),
							1 => *new_conn_count += 1,
							_ => *new_neuron_count += 1
						}
					} else {
						// Sometimes shrink weight (which can effectively remove)
						Neuron::expand_or_shrink(&mut conn.weight, -1.0)
					}
				}
			}
		}

		// Remove effectively dead connections
		self.next_conn.retain(|conn| (conn.weight*10.0).round() != 0.0);

		// If this neuron is inactive, can be recycled
		if self.next_conn.len() < 1 && *new_neuron_count > 0 {
			*new_neuron_count -= 1;
			self.next_conn.push(OutwardConn::new(recv_neuron_count))
		}

		// Reset excitation
		self.excitation = 0.0;

		// Assume not reachable until proven otherwise
		self.reachable = false
	}

	fn drain(&mut self) {
		Neuron::expand_or_shrink(&mut self.excitation, -self.tick_drain.abs())
	}

	fn expand_or_shrink(state: &mut f64, change: f64) {
		// Move towards or away from a neutral state of 0
		if *state > 0.0 {
			*state += change;
			if *state < 0.0 {
				*state = 0.0
			}
		} else {
			*state -= change;
			if *state > 0.0 {
				*state = 0.0
			}
		}
	}
}

impl OutwardConn {
	fn new(recv_neuron_count: usize) -> OutwardConn {
		OutwardConn {
			dest_index: rand_range(0..recv_neuron_count),
			speed: 0,
			weight: [-1.0, 1.0][rand_range(0..=1)]
		}
	}
}

impl fmt::Debug for Brain {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = String::from("Brain {\n");

		s += "\tneurons_in: [\n";
		for neuron in &self.neurons_in {
			s += &format!("\t\t{neuron:#?},\n")
		}

		let (mut unreachables, mut inactives) = (0, 0);

		s += "\t],\n\n\tneurons_hidden: [\n";
		for neuron in &self.neurons_hidden {
			if neuron.reachable {
				s += &format!("\t\t{neuron:#?},\n")
			} else {
				unreachables += 1;
				if neuron.next_conn.len() < 1 {
					inactives += 1
				}
			}
		}
		s += &format!("\n\t\tUNREACHABLES: {unreachables}, INACTIVES: {inactives}\n");

		s += "\t],\n\n\tneurons_out: [\n";
		for neuron in &self.neurons_out {
			s += &format!("\t\t{neuron:#?},\n")
		}

		write!(f, "{s}\t],\n\n\tgeneration: {},\n}}", self.generation)
	}
}

impl fmt::Debug for Neuron {
	// Print neuron debug info in a concise way
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if !self.reachable {
			if self.next_conn.len() < 1 {
				write!(f, "Neuron {{UNREACHABLE & INACTIVE}}")
			} else {
				write!(f, "Neuron {{UNREACHABLE, conns={}}}", self.next_conn.len())
			}
		} else if self.next_conn.len() < 1 {
			write!(f, "Neuron {{INACTIVE}}")
		} else {
			let (is_at, act_at) = (self.excitation, self.act_threshold);
			let mut s = format!("Neuron {{IS@{:.1} | ACT@{:.1} | ", is_at, act_at);

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
