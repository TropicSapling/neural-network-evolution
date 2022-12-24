use crate::helpers::*;

#[derive(Debug)]
pub struct Agent {
	pub brain : Brain,
	pub body  : Body
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
	pub excitation: isize,
	pub tick_drain: usize,

	pub act_threshold: usize,

	pub next_conn: Vec<ForwardConn>
}

#[derive(Debug)]
pub struct ForwardConn {
	pub dest_index: usize,
	pub speed: usize,
	pub weight: isize
}

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

#[derive(Debug)]
pub struct Pos {pub x: f64, pub y: f64}

////////////////////////////////

impl Agent {
	pub fn new() -> Agent {
		Agent {
			brain: Brain {
				neurons_in     : [Neuron::new(), Neuron::new(), Neuron::new()],
				neurons_hidden : vec![],
				neurons_out    : [Neuron::new(), Neuron::new()]
			},

			body: Body {
				colour  : Colour::new(),
				pos     : Pos::new(),
				size    : rand_range(64.0..128.0),
				angle   : 0.0,

				moving  : true,
				turning : true
			}
		}
	}
}

impl Neuron {
	fn new() -> Neuron {
		Neuron {
			excitation: 0,
			tick_drain: 0,

			act_threshold: 1,

			next_conn: vec![]
		}
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
