use crate::helpers::*;

const OUTS: usize = 2;

#[derive(Debug)]
pub struct Agent {
	pub brain : Brain,
	pub body  : Body,
	pub alive : bool
}

////////////////////////////////

/// neurons_in  : [x, y, size] normalised to [0, 1]
/// neurons_out : [moving, turning]
#[derive(Debug)]
pub struct Brain {
	pub neurons_in     : [Neuron; 3],
	pub neurons_hidden : Vec<Neuron>,
	pub neurons_out    : [Neuron; 2]
}

#[derive(Debug)]
pub struct Neuron {
	pub excitation: usize,
	pub tick_drain: usize,

	pub act_threshold: usize,

	pub next_conn: Vec<ForwardConn>
}

#[derive(Clone, Debug)]
pub struct ForwardConn {
	pub dest_index: usize,
	pub speed: usize,
	pub weight: isize
}

// TODO:
// Strengthen/weaken connection weight if receiving neuron
// activates shortly after/before connection fired.

////////////////////////////////

#[derive(Debug)]
pub struct Body {
	pub colour: Colour,

	pub pos   : Pos,
	pub size  : f64,
	pub angle : f64,

	pub moving  : bool,
	pub turning : bool
}

#[derive(Debug)]
pub struct Colour {
	pub r: usize,
	pub g: usize,
	pub b: usize
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {pub x: f64, pub y: f64}

////////////////////////////////

impl Agent {
	pub fn new() -> Agent {
		Agent {
			brain: Brain {
				neurons_in     :     [Neuron::new(3), Neuron::new(3), Neuron::new(3)],
				neurons_hidden : vec![                Neuron::new(3)                ],
				neurons_out    :     [Neuron::new(3),                 Neuron::new(3)]
			},

			body: Body {
				colour  : Colour::new(),
				pos     : Pos::new(),
				size    : rand_range(64.0..96.0),
				angle   : rand_range(0.0..360.0),

				moving  : true,
				turning : true
			},

			alive: true
		}
	}
}

impl Brain {
	pub fn update_neurons(&mut self) -> &[Neuron; 2] {
		// Just for testing
		self.neurons_in[0].excitation = rand_range(0..=1);
		self.neurons_in[1].excitation = rand_range(0..=1);
		self.neurons_in[2].excitation = rand_range(0..=1);

		for i in 0..self.neurons_hidden.len() {
			let neuron = &self.neurons_hidden[i];

			// If neuron activated...
			if neuron.excitation >= neuron.act_threshold {
				// ... prepare all connections for activation
				let mut activations = vec![];
				for conn in &neuron.next_conn {
					activations.push(conn.clone());
				}

				// ... and then activate the connections
				for conn in activations {
					let recv_neuron = if conn.dest_index < OUTS {
						&mut self.neurons_out[conn.dest_index]
					} else {
						&mut self.neurons_hidden[conn.dest_index - OUTS]
					};

					if conn.weight < 0 {
						recv_neuron.inhibit(-conn.weight as usize)
					} else {
						recv_neuron.excite(conn.weight as usize)
					}
				}
			}

			let neuron = &mut self.neurons_hidden[i];
			if neuron.excitation >= neuron.tick_drain {
				neuron.excitation -= neuron.tick_drain
			}
		}

		&self.neurons_out
	}
}

impl Neuron {
	fn new(recv_neuron_count: usize) -> Neuron {
		Neuron {
			excitation: 0,
			tick_drain: 0,

			act_threshold: 1,

			next_conn: vec![ForwardConn {
				dest_index: rand_range(0..recv_neuron_count),
				speed: 0,
				weight: 1
			}]
		}
	}

	fn inhibit(&mut self, amount: usize) {
		if self.excitation > amount {
			self.excitation -= amount
		} else {
			self.excitation = 0
		}
	}

	fn excite(&mut self, amount: usize) {
		self.excitation += amount
	}
}

////////////////////////////////

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
