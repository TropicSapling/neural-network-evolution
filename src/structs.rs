pub struct Agent {
	pub neurons: Vec<Neuron>,
	pub colour: Colour,
	pub pos: Pos,
	pub size: usize,
	pub angle: f64,

	pub moving: bool,
	pub turning: bool
}

pub struct Colour {pub r: usize, pub g: usize, pub b: usize}
pub struct Pos    {pub x: f64, pub y: f64}

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
