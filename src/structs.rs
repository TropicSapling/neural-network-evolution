pub struct Agent {
	pub neurons: Vec<Neuron>,
	pub colour: Colour,
	pub pos: Pos,
	pub size: usize
}

pub struct Colour {pub r: usize, pub g: usize, pub b: usize}
pub struct Pos    {pub x: usize, pub y: usize}

pub struct Neuron {
	excitation: isize,
	tick_drain: usize,

	act_threshold: usize,

	next_conn: Vec<ForwardConn>
}

struct ForwardConn {
	dest_index: usize,
	speed: usize,
	weight: isize
}
