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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Colour {
	pub r: usize,
	pub g: usize,
	pub b: usize
}

#[derive(Clone, Copy, Debug)]
pub struct Pos {pub x: f64, pub y: f64}

////////////////////////////////

impl Agent {
	pub fn new(agents: &mut Vec<Agent>) -> Agent {
		for parent in agents {
			if parent.body.size > 80.0 {
				// Spawn child agent

				parent.body.remove(56.0); // shrink parent

				let colour         = parent.body.colour.clone();
				let neurons_hidden = parent.brain.neurons_hidden.clone();

				return Agent::with(neurons_hidden, colour, 56.0).mutate()
			}
		}

		// Spawn new random agent
		Agent::with(vec![Neuron::new(3)], Colour::new(), rand_range(48.0..96.0))
	}

	fn with(neurons_hidden: Vec<Neuron>, colour: Colour, size: f64) -> Agent {
		Agent {
			brain: Brain {
				neurons_in      : [Neuron::new(3), Neuron::new(3), Neuron::new(3)],
				neurons_hidden,
				neurons_out     : [Neuron::new(3), Neuron::new(3)]
			},

			body: Body {
				colour,
				pos     : Pos::new(),
				size,
				angle   : rand_range(0.0..360.0),

				moving  : false,
				turning : false
			},

			alive: true
		}
	}

	fn mutate(mut self) -> Self {
		self.body.colour.r.add_bounded_max(rand_range(-16..16), 256);
		self.body.colour.g.add_bounded_max(rand_range(-16..16), 256);
		self.body.colour.b.add_bounded_max(rand_range(-16..16), 256);

		// TODO: mutate brain

		self
	}
}

impl Brain {
	pub fn update_neurons(&mut self) -> &[Neuron; 2] {
		// Just for testing
		self.neurons_in[0].excitation = rand_range(0..=1);
		self.neurons_in[1].excitation = rand_range(0..=1);
		self.neurons_in[2].excitation = rand_range(0..=1);

		// Drain output neurons from previous excitation
		for i in 0..OUTS {
			let neuron = &mut self.neurons_out[i];
			if neuron.excitation >= neuron.tick_drain {
				neuron.excitation -= neuron.tick_drain
			}
		}

		for i in 0..self.neurons_in.len() {
			self.update_neuron(i, true)
		}

		for i in 0..self.neurons_hidden.len() {
			self.update_neuron(i, false);

			// Drain neuron
			let neuron = &mut self.neurons_hidden[i];
			if neuron.excitation >= neuron.tick_drain {
				neuron.excitation -= neuron.tick_drain
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
				activations.push(conn.clone());
			}

			// ... and then activate the connections
			for conn in activations {
				let recv_neuron = if conn.dest_index < OUTS {
					&mut self.neurons_out[conn.dest_index]
				} else {
					&mut self.neurons_hidden[conn.dest_index - OUTS]
				};

				recv_neuron.excitation.add_bounded(conn.weight)
			}
		}
	}
}

impl Neuron {
	fn new(recv_neuron_count: usize) -> Neuron {
		Neuron {
			excitation: 0,
			tick_drain: 1,

			act_threshold: 1,

			next_conn: vec![ForwardConn {
				dest_index: rand_range(0..recv_neuron_count),
				speed: 0,
				weight: 1
			}]
		}
	}
}

////////////////////////////////

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
