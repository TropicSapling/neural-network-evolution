use std::{ops::Range, cmp::PartialOrd};
use rand::{Rng, distributions::uniform::SampleUniform};

pub fn rand_range<T: SampleUniform + PartialOrd>(range: Range<T>) -> T {
	rand::thread_rng().gen_range(range)
}

pub fn lt<T: std::cmp::PartialOrd>(first: (T, T), second: (T, T)) -> bool {
	first.0 < second.0 && first.1 < second.1
}

pub fn gt<T: std::cmp::PartialOrd>(first: (T, T), second: (T, T)) -> bool {
	first.0 > second.0 && first.1 > second.1
}
