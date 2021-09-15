use num_traits::{FromPrimitive, Zero};
use std::{
	any::type_name,
	collections::VecDeque,
	marker::{self, PhantomData},
	ops::{Add, AddAssign, Div, SubAssign},
};

use super::MovingAverage;

pub struct NoSumMovingAverage<Divisor, Sample> {
	samples: VecDeque<Sample>,
	max_num_samples: usize,
	zero: Sample,
	_marker: marker::PhantomData<Divisor>,
}

impl<Divisor, Sample> MovingAverage<Divisor, Sample> for NoSumMovingAverage<Divisor, Sample>
where
	Sample: Copy
		+ PartialOrd
		+ Add<Output = Sample>
		+ AddAssign
		+ SubAssign
		+ Div<Divisor, Output = Sample>,
	Divisor: FromPrimitive,
{
	fn add_sample(&mut self, new_sample: Sample) {
		if self.max_num_samples == 0 {
			return;
		}

		if self.samples.len() == self.max_num_samples {
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

impl<Divisor, Sample: Zero> NoSumMovingAverage<Divisor, Sample> {
	pub fn new(max_num_samples: usize) -> Self {
		Self {
			samples: VecDeque::with_capacity(max_num_samples),
			max_num_samples,
			zero: Sample::zero(),
			_marker: PhantomData,
		}
	}
}

impl<Divisor, Sample> NoSumMovingAverage<Divisor, Sample> {
	pub fn from_zero(zero: Sample, max_num_samples: usize) -> Self {
		Self {
			samples: VecDeque::with_capacity(max_num_samples),
			max_num_samples,
			zero,
			_marker: PhantomData,
		}
	}
}
