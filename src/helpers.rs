use std::{ops::RangeBounds, cmp::PartialOrd};
use rand::{Rng, distributions::uniform::{SampleRange, SampleUniform}};

pub fn rand_range<T: SampleUniform + PartialOrd, R: RangeBounds<T> + SampleRange<T>>(range: R) -> T {
	rand::thread_rng().gen_range(range)
}
