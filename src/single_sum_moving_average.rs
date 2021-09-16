use num_traits::{FromPrimitive, Zero};
use std::{
	any::type_name,
	collections::VecDeque,
	marker::{self, PhantomData},
	ops::{AddAssign, Div, SubAssign},
};

use super::MovingAverage;

pub struct SingleSumMovingAverage<Divisor, Sample, const MAX_NUM_SAMPLES: usize> {
	samples: VecDeque<Sample>,
	sum: Sample,
	_marker: marker::PhantomData<Divisor>,
}

impl<Divisor, Sample, const MAX_NUM_SAMPLES: usize> MovingAverage<Divisor, Sample>
	for SingleSumMovingAverage<Divisor, Sample, MAX_NUM_SAMPLES>
where
	Sample: Copy + AddAssign + SubAssign + Div<Divisor, Output = Sample>,
	Divisor: FromPrimitive,
{
	fn add_sample(&mut self, new_sample: Sample) {
		if MAX_NUM_SAMPLES == 0 {
			return;
		}

		self.sum += new_sample;

		if self.samples.len() == MAX_NUM_SAMPLES {
			self.sum -= self.samples.pop_back().unwrap_or(self.sum);
		}

		self.samples.push_front(new_sample);
	}

	fn get_num_samples(&self) -> usize {
		self.samples.len()
	}

	fn get_average_sample(&self) -> Sample {
		let num_samples = self.samples.len();

		if num_samples == 0 {
			return self.sum;
		}

		let num_samples = Divisor::from_usize(num_samples).unwrap_or_else(|| {
			panic!(
				"Failed to create a divisor of type {} from num_samples: usize = {}",
				type_name::<Divisor>(),
				num_samples
			)
		});
		self.sum / num_samples
	}

	fn get_most_recent_sample(&self) -> Option<Sample> {
		self.samples.front().cloned()
	}

	fn get_samples(&mut self) -> &[Sample] {
		self.samples.make_contiguous();
		self.samples.as_slices().0
	}
}

impl<Divisor, Sample: Zero, const MAX_NUM_SAMPLES: usize>
	SingleSumMovingAverage<Divisor, Sample, MAX_NUM_SAMPLES>
{
	pub fn new() -> Self {
		Self {
			samples: VecDeque::with_capacity(MAX_NUM_SAMPLES),
			sum: Sample::zero(),
			_marker: PhantomData,
		}
	}
}

impl<Divisor, Sample, const MAX_NUM_SAMPLES: usize>
	SingleSumMovingAverage<Divisor, Sample, MAX_NUM_SAMPLES>
{
	pub fn from_zero(zero: Sample) -> Self {
		Self {
			samples: VecDeque::with_capacity(MAX_NUM_SAMPLES),
			sum: zero,
			_marker: PhantomData,
		}
	}
}
