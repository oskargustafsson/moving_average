use num_traits::{FromPrimitive, Zero};
use std::{
	any::type_name,
	collections::VecDeque,
	marker::{self, PhantomData},
	ops::{Add, Div},
};

use super::{sum_tree::SumTree, MovingAverage};

type SumTreeNodeIdx = usize;

pub struct SumTreeMovingAverage<Sample, Divisor, const WINDOW_SIZE: usize> {
	samples: VecDeque<SumTreeNodeIdx>,
	sum_tree: SumTree<Sample>,
	_marker: marker::PhantomData<Divisor>,
}

impl<Sample, Divisor, const WINDOW_SIZE: usize> MovingAverage<Sample, Divisor>
	for SumTreeMovingAverage<Sample, Divisor, WINDOW_SIZE>
where
	Sample: Copy + Add<Output = Sample> + Div<Divisor, Output = Sample>,
	Divisor: FromPrimitive,
{
	fn add_sample(&mut self, new_sample: Sample) {
		if WINDOW_SIZE == 0 {
			return;
		}

		let tree_node_idx = if self.samples.len() < WINDOW_SIZE {
			self.samples.len()
		} else {
			self.samples.pop_back().unwrap()
		};

		self.sum_tree
			.update_leaf_node_sample(tree_node_idx, new_sample);
		self.samples.push_front(tree_node_idx);
	}

	fn get_num_samples(&self) -> usize {
		self.samples.len()
	}

	fn get_average_sample(&self) -> Sample {
		let num_samples = self.samples.len();

		if num_samples == 0 {
			return self.sum_tree.get_root_sum();
		}

		let num_samples = Divisor::from_usize(num_samples).unwrap_or_else(|| {
			panic!(
				"Failed to create a divisor of type {} from num_samples: usize = {}",
				type_name::<Divisor>(),
				num_samples
			)
		});
		self.sum_tree.get_root_sum() / num_samples
	}

	fn get_most_recent_sample(&self) -> Option<Sample> {
		self.samples
			.front()
			.map(|node_idx| self.sum_tree.get_leaf_node_sum(node_idx))
	}

	fn get_samples(&mut self) -> &[Sample] {
		self.sum_tree.get_leaf_nodes_slice()
	}

	fn get_sample_window_size(&self) -> usize {
		WINDOW_SIZE
	}
}

impl<Sample: Zero + Copy, Divisor, const WINDOW_SIZE: usize>
	SumTreeMovingAverage<Sample, Divisor, WINDOW_SIZE>
{
	pub fn new() -> Self {
		Self {
			samples: VecDeque::with_capacity(WINDOW_SIZE),
			sum_tree: SumTree::new(Sample::zero(), WINDOW_SIZE),
			_marker: PhantomData,
		}
	}
}

impl<Sample: Copy, Divisor, const WINDOW_SIZE: usize>
	SumTreeMovingAverage<Sample, Divisor, WINDOW_SIZE>
{
	pub fn from_zero(zero: Sample) -> Self {
		Self {
			samples: VecDeque::with_capacity(WINDOW_SIZE),
			sum_tree: SumTree::new(zero, WINDOW_SIZE),
			_marker: PhantomData,
		}
	}
}
