use num_traits::{FromPrimitive, Zero};
use std::{
	any::type_name,
	collections::VecDeque,
	marker::{self, PhantomData},
	ops::{Add, AddAssign, Div, SubAssign},
};

use super::{sum_tree::SumTree, MovingAverage};

type SumTreeNodeIdx = usize;

pub struct SumTreeMovingAverage<Divisor, Sample> {
	samples: VecDeque<SumTreeNodeIdx>,
	max_num_samples: usize,
	sum_tree: SumTree<Sample>,
	_marker: marker::PhantomData<Divisor>,
}

impl<Divisor, Sample> MovingAverage<Divisor, Sample> for SumTreeMovingAverage<Divisor, Sample>
where
	Sample: Copy + AddAssign + Add<Output = Sample> + SubAssign + Div<Divisor, Output = Sample>,
	Divisor: FromPrimitive,
{
	fn add_sample(&mut self, new_sample: Sample) {
		if self.max_num_samples == 0 {
			return;
		}

		let tree_node_idx = if self.samples.len() < self.max_num_samples {
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
}

impl<Divisor, Sample: Zero + Copy> SumTreeMovingAverage<Divisor, Sample> {
	pub fn new(max_num_samples: usize) -> Self {
		Self {
			samples: VecDeque::with_capacity(max_num_samples),
			max_num_samples,
			sum_tree: SumTree::new(Sample::zero(), max_num_samples),
			_marker: PhantomData,
		}
	}
}

impl<Divisor, Sample: Copy> SumTreeMovingAverage<Divisor, Sample> {
	pub fn from_zero(zero: Sample, max_num_samples: usize) -> Self {
		Self {
			samples: VecDeque::with_capacity(max_num_samples),
			max_num_samples,
			sum_tree: SumTree::new(zero, max_num_samples),
			_marker: PhantomData,
		}
	}
}
