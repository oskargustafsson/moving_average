use num_traits::{FromPrimitive, Zero};
use std::{
	any::type_name,
	collections::VecDeque,
	marker::{self, PhantomData},
	ops::{AddAssign, Div},
};

use super::MovingAverage;

pub struct NoSumMovingAverage<Divisor, Sample, const MAX_NUM_SAMPLES: usize> {
	samples: VecDeque<Sample>,
	zero: Sample,
	_marker: marker::PhantomData<Divisor>,
}

impl<Divisor, Sample, const MAX_NUM_SAMPLES: usize> MovingAverage<Divisor, Sample>
	for NoSumMovingAverage<Divisor, Sample, MAX_NUM_SAMPLES>
where
	Sample: Copy + AddAssign + Div<Divisor, Output = Sample>,
	Divisor: FromPrimitive,
{
	fn add_sample(&mut self, new_sample: Sample) {
		if MAX_NUM_SAMPLES == 0 {
			return;
		}

		if self.samples.len() == MAX_NUM_SAMPLES {
			self.samples.pop_back();
		}

		self.samples.push_front(new_sample);
	}

	fn get_num_samples(&self) -> usize {
		self.samples.len()
	}

	fn get_average_sample(&self) -> Sample {
		let num_samples = self.samples.len();

		if num_samples == 0 {
			return self.zero;
		}

		let num_samples = Divisor::from_usize(num_samples).unwrap_or_else(|| {
			panic!(
				"Failed to create a divisor of type {} from num_samples: usize = {}",
				type_name::<Divisor>(),
				num_samples
			)
		});

		let sum = {
			let mut sum = self.zero;
			for sample in &self.samples {
				sum += *sample;
			}
			sum
		};

		sum / num_samples
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
	NoSumMovingAverage<Divisor, Sample, MAX_NUM_SAMPLES>
{
	pub fn new() -> Self {
		Self {
			samples: VecDeque::with_capacity(MAX_NUM_SAMPLES),
			zero: Sample::zero(),
			_marker: PhantomData,
		}
	}
}

impl<Divisor, Sample, const MAX_NUM_SAMPLES: usize>
	NoSumMovingAverage<Divisor, Sample, MAX_NUM_SAMPLES>
{
	pub fn from_zero(zero: Sample) -> Self {
		Self {
			samples: VecDeque::with_capacity(MAX_NUM_SAMPLES),
			zero,
			_marker: PhantomData,
		}
	}
}
