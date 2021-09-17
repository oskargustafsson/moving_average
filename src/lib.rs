/*!
This crate provides several algorithms for calculating
[simple moving averages](https://en.wikipedia.org/wiki/Moving_average#Simple_moving_averages).

All variants implement the MovingAverage trait, which provides a unified iterface. The interface
is generic over sample type, meaning that any type that supports addition and division by a scalar
can be averaged. This includes most primitive numeric types
([f32](https://doc.rust-lang.org/std/primitive.f32.html),
[u32](https://doc.rust-lang.org/std/primitive.u32.html), ...),
[Duration](https://doc.rust-lang.org/std/time/struct.Duration.html) and
many third party math library ([nalgebra](https://docs.rs/nalgebra/),
[euclid](https://docs.rs/euclid/), [cgmath](https://docs.rs/cgmath/), ...) types.

## Examples

*Floating point numbers*
```rust
# use moving_average::{MovingAverage, SumTreeMovingAverage};
let mut ma = SumTreeMovingAverage::<_, f32, 2>::new(); // Window size = 2
ma.add_sample(1.0);
ma.add_sample(2.0);
ma.add_sample(3.0);
assert_eq!(ma.get_average_sample(), 2.5); // (2 + 3) / 2 = 2.5
```

*Durations*
```rust
# use moving_average::{MovingAverage, SingleSumMovingAverage};
# use std::time::{Duration, Instant};
let mut ma = SingleSumMovingAverage::<_, _, 10>::from_zero(Duration::ZERO);
loop {
	let instant = Instant::now();
	// [ application code ]
	ma.add_sample(instant.elapsed());
	dbg!("Average iteration duration: {}", ma.get_average_sample());
	# break;
}
```

## Implementations

One way to achieve good performance when calculating simple moving averages is to cache previous
calculations, specifically the sum of the samples currently in the sample window. Caching this sum
has both pros and cons, which is what motivates the three different implementations described below.

### SingleSumMovingAverage

This implementation caches the sum of all samples in the sample window as a single value, leading to
`O(1)` time complexity for both writing new samples and reading their average. The problem with this
approach is that most floating point numbers can't be stored exactly, so every time a such a number
is added to the sum, there is a risk of accumulating a rounding error.

Thankfully, the mean signed rounding error (`summed_average - exact_average`) for floating point
numbers is 0, as the numbers are on average rounded up as much as they are rounded down. The
variance of the signed error, however increases with the number of samples added, analogously to a
[random walk](https://stats.stackexchange.com/questions/159650/why-does-the-variance-of-the-random-walk-increase).

The magnitude of the error variance depends on many factors, including moving window size, average
sample magnitude and distribution. Below is a visualization of how the absolute difference in
average value between SingleSumMovingAverage and NoSumMovingAverage (which does not suffer from
accumulated rounding errors) grows with the number of samples, for a typical window and set of
samples.

`Sample window size: 10`

`Sample distribution: Uniform in range [-100.0, 100.0]`

`Test runs: 100`

![Difference between SingleSumMovingAverage and NoSumMovingAverage](https://raw.githubusercontent.com/oskargustafsson/moving_average/master/res/single_sum_diff.png)


*Note that both axes of the graph are logarithmic.*

**When to use**
 - When samples can be represented exactly in memory, in which case there is no downside to this
   approach. Such samples include all integer types and
   [Duration](https://doc.rust-lang.org/std/time/struct.Duration.html), which is backed by integer
   types.
 - When performance is more important than numerical accuracy.

### NoSumMovingAverage

The error variance issue described above can be avoided by simply not caching the sum and
calculating it from scratch, at `O(N)` time complexity, every time it is requested. This is what
NoSumMovingAverage does.

**When to use**
 - When the sample window size is so small that the samples summation cost is negligable.
 - When new samples are written significantly more often than the average value is read.

### SumTreeMovingAverage

There is a way of avoiding the accumulated floating point rounding errors, without having to
re-calculate the whole sum every time the average value is requested. The downside though, is that
it involves both binary trees and math:

A sum is the result of applying the binary and
[associative](https://en.wikipedia.org/wiki/Associative_property)
addition operation to a set of operands, which means that it can be represented as a binary tree of
sums.

For example

`(1) + (2) + (3) + (4) + (5) + (6)` =

`(1 + 2) + (3 + 4) + (5 + 6)` =

`(3) + (7) + (11)` =

`(3 + 7) + (11)` =

`(10) + (11)` =

`(10 + 11)` =

`(21)`

can be represented as the following tree.
```text
# Note to self: Each line in the ASCII art below starts with a "Zero width non-joiner" to stop
# rustfmt from converting the subsequent spaces to tabs.
‌           21
‌          /  \
‌         /    \
‌       10      11
‌      /  \      \
‌     /    \      \
‌    3      7      11
‌   / \    / \    /  \
‌  1   2  3   4  5    6
```

If one of the leaf nodes (i.e. our samples) were to change, only the nodes comprising the direct
path between that leaf and the root need to be re-calculated, leading to `log(N)` calculations, `N`
being the window size. This is exactly what happens when a sample is added; the oldest sample gets
replaced with the new sample and sum tree leaf node corresponding to the oldest sample is updated
with the new sample value.

One existing leaf node (i.e. sample value) is always re-read when updating that leaf node's
neighbour, meaning that after N samples have been added, all the leaf nodes have been re-read. This
is what keeps the floating point rounding error from accumulating.

*Author's note:* If anyone has the brains and will to prove this formally, they are most welcome to
submit a [PR](https://github.com/oskargustafsson/moving_average/pulls). In the mean time, there is a
unit test that empirically proves that the rounding error does not accumulate. Part of that test's
output data is visualized in the graph below, showing no accumulated rounding errors when compared
with NoSumMovingAverage.

![Difference between SumTreeMovingAverage and NoSumMovingAverage](https://raw.githubusercontent.com/oskargustafsson/moving_average/master/res/sum_tree_diff.png)

**When to use**
 - In most cases where floating point data is involved, unless writes are much more common than
   reads.

### Summary (no pun intended)

| Implementation         | Add sample   | Get average   |
|------------------------|--------------|---------------|
| SingleSumMovingAverage | `O(1)`       | `O(1)`        |
| NoSumMovingAverage     | `O(1)`       | `O(N)`        |
| SumTreeMovingAverage   | `O(log(N))`  | `O(1)`        |

`N` refers to the size of the sample window.

*/

mod moving_average;
mod no_sum_moving_average;
mod single_sum_moving_average;
mod sum_tree;
mod sum_tree_moving_average;

pub use crate::moving_average::MovingAverage;
pub use crate::no_sum_moving_average::NoSumMovingAverage;
pub use crate::single_sum_moving_average::SingleSumMovingAverage;
pub use crate::sum_tree_moving_average::SumTreeMovingAverage;

#[cfg(test)]
mod tests {
	use crate::{MovingAverage, NoSumMovingAverage, SingleSumMovingAverage, SumTreeMovingAverage};

	macro_rules! get_ma_impls {
		(
			$divisor_type:ty, $window_size:expr, $ctor:ident $(, $zero:expr)?
		) => {{
			let ma_impls: [Box<dyn MovingAverage<_, $divisor_type>>; 3] = [
				Box::new(SingleSumMovingAverage::<_, _, $window_size>::$ctor($($zero ,)?)),
				Box::new(SumTreeMovingAverage::<_, _, $window_size>::$ctor($($zero ,)?)),
				Box::new(NoSumMovingAverage::<_, _, $window_size>::$ctor($($zero ,)?)),
			];
			ma_impls
		}};
	}

	#[test]
	fn f32_samples() {
		for ma in &mut get_ma_impls!(f32, 3, new) {
			assert_eq!(ma.get_average(), 0.0);
			assert_eq!(ma.get_num_samples(), 0);

			ma.add_sample(4.0);
			assert_eq!(ma.get_average(), 4.0);
			assert_eq!(ma.get_num_samples(), 1);

			ma.add_sample(8.0);
			assert_eq!(ma.get_average(), 6.0);
			assert_eq!(ma.get_num_samples(), 2);

			ma.add_sample(3.0);
			assert_eq!(ma.get_average(), 5.0);
			assert_eq!(ma.get_num_samples(), 3);

			// Here we reach window_size and start to pop old samples

			ma.add_sample(7.0);
			assert_eq!(ma.get_average(), 6.0);
			assert_eq!(ma.get_num_samples(), 3);

			ma.add_sample(11.0);
			assert_eq!(ma.get_average(), 7.0);
			assert_eq!(ma.get_num_samples(), 3);

			ma.add_sample(0.0);
			assert_eq!(ma.get_average(), 6.0);
			assert_eq!(ma.get_num_samples(), 3);

			ma.add_sample(-23.0);
			assert_eq!(ma.get_average(), -4.0);
			assert_eq!(ma.get_num_samples(), 3);
		}
	}

	#[test]
	fn u32_samples() {
		for ma in &mut get_ma_impls!(u32, 3, new) {
			assert_eq!(ma.get_average(), 0);

			ma.add_sample(4);
			assert_eq!(ma.get_average(), 4);

			ma.add_sample(8);
			assert_eq!(ma.get_average(), 6);

			ma.add_sample(3);
			assert_eq!(ma.get_average(), 5);

			ma.add_sample(7);
			assert_eq!(ma.get_average(), 6);

			ma.add_sample(11);
			assert_eq!(ma.get_average(), 7);

			ma.add_sample(0);
			assert_eq!(ma.get_average(), 6);
		}
	}

	#[test]
	fn u32_samples_2() {
		for ma in &mut get_ma_impls!(u32, 3, new) {
			ma.add_sample(1);
			assert_eq!(ma.get_average(), 1);

			ma.add_sample(2);
			assert_eq!(ma.get_average(), 1);

			ma.add_sample(3);
			assert_eq!(ma.get_average(), 2);

			ma.add_sample(4);
			assert_eq!(ma.get_average(), 3);

			ma.add_sample(10);
			assert_eq!(ma.get_average(), 5);
		}
	}

	#[test]
	fn nalgebra_vector2_f32_samples() {
		use nalgebra::Vector2;

		for ma in &mut get_ma_impls!(f32, 3, new) {
			assert_eq!(ma.get_average(), Vector2::new(0.0, 0.0));

			ma.add_sample(Vector2::new(4.0, 8.0));
			assert_eq!(ma.get_average(), Vector2::new(4.0, 8.0));

			ma.add_sample(Vector2::new(6.0, 0.0));
			assert_eq!(ma.get_average(), Vector2::new(5.0, 4.0));

			ma.add_sample(Vector2::new(2.0, 10.0));
			assert_eq!(ma.get_average(), Vector2::new(4.0, 6.0));

			ma.add_sample(Vector2::new(-17.0, 20.0));
			assert_eq!(ma.get_average(), Vector2::new(-3.0, 10.0));

			ma.add_sample(Vector2::new(0.0, -21.0));
			assert_eq!(ma.get_average(), Vector2::new(-5.0, 3.0));
		}
	}

	#[test]
	fn euclid_vector2_f32_samples() {
		use euclid::default::Vector2D;

		for ma in &mut get_ma_impls!(f32, 3, from_zero, Vector2D::zero()) {
			assert_eq!(ma.get_average(), Vector2D::new(0.0, 0.0));

			ma.add_sample(Vector2D::new(4.0, 8.0));
			assert_eq!(ma.get_average(), Vector2D::new(4.0, 8.0));

			ma.add_sample(Vector2D::new(6.0, 0.0));
			assert_eq!(ma.get_average(), Vector2D::new(5.0, 4.0));

			ma.add_sample(Vector2D::new(2.0, 10.0));
			assert_eq!(ma.get_average(), Vector2D::new(4.0, 6.0));

			ma.add_sample(Vector2D::new(-17.0, 20.0));
			assert_eq!(ma.get_average(), Vector2D::new(-3.0, 10.0));

			ma.add_sample(Vector2D::new(0.0, -21.0));
			assert_eq!(ma.get_average(), Vector2D::new(-5.0, 3.0));
		}
	}

	#[test]
	fn cgmath_vector2_f32_samples() {
		use cgmath::Vector2;

		for ma in &mut get_ma_impls!(f32, 3, new) {
			assert_eq!(ma.get_average(), Vector2::new(0.0, 0.0));

			ma.add_sample(Vector2::new(4.0, 8.0));
			assert_eq!(ma.get_average(), Vector2::new(4.0, 8.0));

			ma.add_sample(Vector2::new(6.0, 0.0));
			assert_eq!(ma.get_average(), Vector2::new(5.0, 4.0));

			ma.add_sample(Vector2::new(2.0, 10.0));
			assert_eq!(ma.get_average(), Vector2::new(4.0, 6.0));

			ma.add_sample(Vector2::new(-17.0, 20.0));
			assert_eq!(ma.get_average(), Vector2::new(-3.0, 10.0));

			ma.add_sample(Vector2::new(0.0, -21.0));
			assert_eq!(ma.get_average(), Vector2::new(-5.0, 3.0));
		}
	}

	#[test]
	fn duration_samples() {
		use std::time::Duration;

		for ma in &mut get_ma_impls!(u32, 3, from_zero, Duration::ZERO) {
			assert_eq!(ma.get_average(), Duration::from_secs(0));

			ma.add_sample(Duration::from_secs(10));
			assert_eq!(ma.get_average(), Duration::from_secs(10));

			ma.add_sample(Duration::from_secs(20));
			assert_eq!(ma.get_average(), Duration::from_secs(15));

			ma.add_sample(Duration::from_secs(30));
			assert_eq!(ma.get_average(), Duration::from_secs(20));

			ma.add_sample(Duration::from_secs(1));
			assert_eq!(ma.get_average(), Duration::from_secs(17));

			ma.add_sample(Duration::from_secs(32));
			assert_eq!(ma.get_average(), Duration::from_secs(21));
		}
	}

	#[test]
	fn edge_case_zero_sized() {
		for ma in &mut get_ma_impls!(u32, 0, new) {
			assert_eq!(ma.get_average(), 0);
			assert_eq!(ma.get_num_samples(), 0);

			ma.add_sample(16);
			assert_eq!(ma.get_average(), 0);
			assert_eq!(ma.get_num_samples(), 0);
		}
	}

	#[test]
	fn f32_random_samples_max_algorithm_diffs() {
		use rand::{distributions::Uniform, rngs::SmallRng, Rng, SeedableRng};

		const WINDOW_SIZE: usize = 10;

		const VALUE_RANGES: [(usize, usize); 6] = [
			(0, 10),
			(10, 100),
			(100, 1000),
			(1000, 10000),
			(10000, 100000),
			(100000, 1000000),
		];

		let seeds: Vec<u64> = SmallRng::seed_from_u64(0xCAFEBABE)
			.sample_iter(&Uniform::from(0..u64::MAX))
			.take(100)
			.collect();

		let averages_array_array: Vec<[[f32; 3]; VALUE_RANGES.len()]> = seeds
			.iter()
			.map(|seed| {
				let random_values: Vec<f32> = SmallRng::seed_from_u64(*seed)
					.sample_iter(&Uniform::from(-100.0..100.0))
					.take(1000000)
					.collect();

				let mut single_sum_ma = SingleSumMovingAverage::<_, f32, WINDOW_SIZE>::new();
				let mut sum_tree_ma = SumTreeMovingAverage::<_, f32, WINDOW_SIZE>::new();
				let mut no_sum_ma = NoSumMovingAverage::<_, f32, WINDOW_SIZE>::new();

				VALUE_RANGES.map(|value_range| {
					for random_value in &random_values[value_range.0..value_range.1] {
						single_sum_ma.add_sample(*random_value);
						sum_tree_ma.add_sample(*random_value);
						no_sum_ma.add_sample(*random_value);
					}
					[
						single_sum_ma.get_average(),
						sum_tree_ma.get_average(),
						no_sum_ma.get_average(),
					]
				})
			})
			.collect();

		let mut maximum_absolute_diffs = [[0.0f32; VALUE_RANGES.len()]; 2];

		for averages_array in averages_array_array {
			for (idx, averages) in averages_array.iter().enumerate() {
				for i in 0..2 {
					let abs_diff = (averages[i] - averages[2]).abs();
					if maximum_absolute_diffs[i][idx] < abs_diff {
						maximum_absolute_diffs[i][idx] = abs_diff;
					}
				}
			}
		}

		let single_sum_maximum_absolute_diff = *maximum_absolute_diffs[0]
			.iter()
			.max_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap())
			.unwrap();

		assert!(single_sum_maximum_absolute_diff < 0.002);

		let sum_tree_maximum_absolute_diff = *maximum_absolute_diffs[1]
			.iter()
			.max_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap())
			.unwrap();

		assert!(sum_tree_maximum_absolute_diff < 0.000005);
	}
}
