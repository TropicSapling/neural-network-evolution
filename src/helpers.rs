use std::{ops::Range, cmp::PartialOrd};
use rand::{Rng, distributions::uniform::SampleUniform};

pub fn rand_range<T: SampleUniform + PartialOrd>(range: Range<T>) -> T {
	rand::thread_rng().gen_range(range)
}
