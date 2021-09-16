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
calculations, specifically the sum of the samples currently in the moving window.

### SingleSumMovingAverage

This is exactly what is done in the SingleSumMovingAverage implementation (hence the name, a single
sum of all samples is cached). The only problem with this approach is that most floating point
numbers can't be stored exactly, so every time a new number is added to the sum, there is a risk of
accumulating a rounding error.

Thankfully, the mean signed rounding error (`summed_average - exact_average`) for floating point
numbers is 0, as the numbers are on average rounded up as much as they are rounded down. The
variance of the signed error, however increases with the number of samples added, analogously to a
[random walk](https://stats.stackexchange.com/questions/159650/why-does-the-variance-of-the-random-walk-increase).

The magnitude of the error variance depends on many factors, including moving window size, average
sample magnitude and distribution. Below is a visualization of how the error variance grows with
the number of samples, for a typical window and set of samples.

TODO: Graph

*Note that both axes of the graph are logarithmic.*

**When to use**
 - For samples whose sum can be exactly represented in memory, e.g. integers
   (including [Duration](https://doc.rust-lang.org/std/time/struct.Duration.html), which is backed
   by integer types).
 - Or, when performance is more important than floating point exactness.

### NoSumMovingAverage

The error variance issue described above can be avoided by simply not caching the sum and
calculating it from scratch, every time it is requested. This is what NoSumMovingAverage does.

### SumTreeMovingAverage

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
unit test that empirically proves that the rounding error does not accumulate.

### Summary (no pun intended)

| Implementation         | Add sample   | Get average   | Caveat |
|------------------------|--------------|---------------|--------|
| SingleSumMovingAverage | `O(1)`       | `O(1)`        | May accumulate floating point rounding errors. |
| NoSumMovingAverage     | `O(1)`       | `O(N)`        |  |
| SumTreeMovingAverage   | `O(log(N))`  | `O(1)`        |  |

`N` in the above chart refers to the sample size of the moving average window.

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
			$divisor_type:ty, $max_num_samples:expr, $ctor:ident $(, $zero:expr)?
		) => {{
			let ma_impls: [Box<dyn MovingAverage<_, $divisor_type>>; 3] = [
				Box::new(SingleSumMovingAverage::<_, _, $max_num_samples>::$ctor($($zero ,)?)),
				Box::new(SumTreeMovingAverage::<_, _, $max_num_samples>::$ctor($($zero ,)?)),
				Box::new(NoSumMovingAverage::<_, _, $max_num_samples>::$ctor($($zero ,)?)),
			];
			ma_impls
		}};
	}

	#[test]
	fn f32_samples() {
		for ma in &mut get_ma_impls!(f32, 3, new) {
			assert_eq!(ma.get_average_sample(), 0.0);
			assert_eq!(ma.get_num_samples(), 0);

			ma.add_sample(4.0);
			assert_eq!(ma.get_average_sample(), 4.0);
			assert_eq!(ma.get_num_samples(), 1);

			ma.add_sample(8.0);
			assert_eq!(ma.get_average_sample(), 6.0);
			assert_eq!(ma.get_num_samples(), 2);

			ma.add_sample(3.0);
			assert_eq!(ma.get_average_sample(), 5.0);
			assert_eq!(ma.get_num_samples(), 3);

			// Here we reach max_num_samples and start to pop old samples

			ma.add_sample(7.0);
			assert_eq!(ma.get_average_sample(), 6.0);
			assert_eq!(ma.get_num_samples(), 3);

			ma.add_sample(11.0);
			assert_eq!(ma.get_average_sample(), 7.0);
			assert_eq!(ma.get_num_samples(), 3);

			ma.add_sample(0.0);
			assert_eq!(ma.get_average_sample(), 6.0);
			assert_eq!(ma.get_num_samples(), 3);

			ma.add_sample(-23.0);
			assert_eq!(ma.get_average_sample(), -4.0);
			assert_eq!(ma.get_num_samples(), 3);
		}
	}

	#[test]
	fn u32_samples() {
		for ma in &mut get_ma_impls!(u32, 3, new) {
			assert_eq!(ma.get_average_sample(), 0);

			ma.add_sample(4);
			assert_eq!(ma.get_average_sample(), 4);

			ma.add_sample(8);
			assert_eq!(ma.get_average_sample(), 6);

			ma.add_sample(3);
			assert_eq!(ma.get_average_sample(), 5);

			ma.add_sample(7);
			assert_eq!(ma.get_average_sample(), 6);

			ma.add_sample(11);
			assert_eq!(ma.get_average_sample(), 7);

			ma.add_sample(0);
			assert_eq!(ma.get_average_sample(), 6);
		}
	}

	#[test]
	fn u32_samples_2() {
		for ma in &mut get_ma_impls!(u32, 3, new) {
			ma.add_sample(1);
			assert_eq!(ma.get_average_sample(), 1);

			ma.add_sample(2);
			assert_eq!(ma.get_average_sample(), 1);

			ma.add_sample(3);
			assert_eq!(ma.get_average_sample(), 2);

			ma.add_sample(4);
			assert_eq!(ma.get_average_sample(), 3);

			ma.add_sample(10);
			assert_eq!(ma.get_average_sample(), 5);
		}
	}

	#[test]
	fn nalgebra_vector2_f32_samples() {
		use nalgebra::Vector2;

		for ma in &mut get_ma_impls!(f32, 3, new) {
			assert_eq!(ma.get_average_sample(), Vector2::new(0.0, 0.0));

			ma.add_sample(Vector2::new(4.0, 8.0));
			assert_eq!(ma.get_average_sample(), Vector2::new(4.0, 8.0));

			ma.add_sample(Vector2::new(6.0, 0.0));
			assert_eq!(ma.get_average_sample(), Vector2::new(5.0, 4.0));

			ma.add_sample(Vector2::new(2.0, 10.0));
			assert_eq!(ma.get_average_sample(), Vector2::new(4.0, 6.0));

			ma.add_sample(Vector2::new(-17.0, 20.0));
			assert_eq!(ma.get_average_sample(), Vector2::new(-3.0, 10.0));

			ma.add_sample(Vector2::new(0.0, -21.0));
			assert_eq!(ma.get_average_sample(), Vector2::new(-5.0, 3.0));
		}
	}

	#[test]
	fn euclid_vector2_f32_samples() {
		use euclid::default::Vector2D;

		for ma in &mut get_ma_impls!(f32, 3, from_zero, Vector2D::zero()) {
			assert_eq!(ma.get_average_sample(), Vector2D::new(0.0, 0.0));

			ma.add_sample(Vector2D::new(4.0, 8.0));
			assert_eq!(ma.get_average_sample(), Vector2D::new(4.0, 8.0));

			ma.add_sample(Vector2D::new(6.0, 0.0));
			assert_eq!(ma.get_average_sample(), Vector2D::new(5.0, 4.0));

			ma.add_sample(Vector2D::new(2.0, 10.0));
			assert_eq!(ma.get_average_sample(), Vector2D::new(4.0, 6.0));

			ma.add_sample(Vector2D::new(-17.0, 20.0));
			assert_eq!(ma.get_average_sample(), Vector2D::new(-3.0, 10.0));

			ma.add_sample(Vector2D::new(0.0, -21.0));
			assert_eq!(ma.get_average_sample(), Vector2D::new(-5.0, 3.0));
		}
	}

	#[test]
	fn cgmath_vector2_f32_samples() {
		use cgmath::Vector2;

		for ma in &mut get_ma_impls!(f32, 3, new) {
			assert_eq!(ma.get_average_sample(), Vector2::new(0.0, 0.0));

			ma.add_sample(Vector2::new(4.0, 8.0));
			assert_eq!(ma.get_average_sample(), Vector2::new(4.0, 8.0));

			ma.add_sample(Vector2::new(6.0, 0.0));
			assert_eq!(ma.get_average_sample(), Vector2::new(5.0, 4.0));

			ma.add_sample(Vector2::new(2.0, 10.0));
			assert_eq!(ma.get_average_sample(), Vector2::new(4.0, 6.0));

			ma.add_sample(Vector2::new(-17.0, 20.0));
			assert_eq!(ma.get_average_sample(), Vector2::new(-3.0, 10.0));

			ma.add_sample(Vector2::new(0.0, -21.0));
			assert_eq!(ma.get_average_sample(), Vector2::new(-5.0, 3.0));
		}
	}

	#[test]
	fn duration_samples() {
		use std::time::Duration;

		for ma in &mut get_ma_impls!(u32, 3, from_zero, Duration::ZERO) {
			assert_eq!(ma.get_average_sample(), Duration::from_secs(0));

			ma.add_sample(Duration::from_secs(10));
			assert_eq!(ma.get_average_sample(), Duration::from_secs(10));

			ma.add_sample(Duration::from_secs(20));
			assert_eq!(ma.get_average_sample(), Duration::from_secs(15));

			ma.add_sample(Duration::from_secs(30));
			assert_eq!(ma.get_average_sample(), Duration::from_secs(20));

			ma.add_sample(Duration::from_secs(1));
			assert_eq!(ma.get_average_sample(), Duration::from_secs(17));

			ma.add_sample(Duration::from_secs(32));
			assert_eq!(ma.get_average_sample(), Duration::from_secs(21));
		}
	}

	#[test]
	fn edge_case_zero_sized() {
		for ma in &mut get_ma_impls!(u32, 0, new) {
			assert_eq!(ma.get_average_sample(), 0);
			assert_eq!(ma.get_num_samples(), 0);

			ma.add_sample(16);
			assert_eq!(ma.get_average_sample(), 0);
			assert_eq!(ma.get_num_samples(), 0);
		}
	}

	// #[test]
	// fn f32_samples_random() {
	// 	// Assert that the error does not grow too much with the number of added samples

	// 	use rand::distributions::Uniform;
	// 	// TODO: Make sure this is just a dev-dependency
	// 	use rand::rngs::SmallRng;
	// 	use rand::{Rng, SeedableRng};

	// 	fn get_exact_average(values: &[f32], at_end_idx: usize, window_size: usize) -> f32 {
	// 		let sum: f32 = values[at_end_idx - window_size..at_end_idx].iter().sum();
	// 		sum / window_size as f32
	// 	}

	// 	let window_size = 10;

	// 	let value_ranges = [
	// 		(0, 10),
	// 		(10, 100),
	// 		(100, 1000),
	// 		(1000, 10000),
	// 		(10000, 100000),
	// 		(100000, 1000000),
	// 	];

	// 	let seeds: Vec<u64> = SmallRng::seed_from_u64(0xCAFEBABE)
	// 		.sample_iter(&Uniform::from(0..u64::MAX))
	// 		.take(10)
	// 		.collect();

	// 	let errors_array: Vec<[f32; 6]> = seeds
	// 		.iter()
	// 		.map(|seed| {
	// 			let random_values: Vec<f32> = SmallRng::seed_from_u64(*seed)
	// 				.sample_iter(&Uniform::from(-10.0..10.0))
	// 				.take(1000000)
	// 				.collect();

	// 			let mut ma = SumTreeMovingAverage::<f32, _>::new(window_size);

	// 			value_ranges.map(|value_range| {
	// 				for random_value in &random_values[value_range.0..value_range.1] {
	// 					ma.add_sample(*random_value);
	// 				}
	// 				let exact_average =
	// 					get_exact_average(&random_values, value_range.1, window_size);
	// 				ma.get_average() - exact_average
	// 			})
	// 		})
	// 		.collect();

	// 	dbg!(&errors_array);

	// 	let mut summed_errors = vec![0.0; value_ranges.len()];

	// 	for errors in errors_array {
	// 		for (idx, error) in errors.iter().enumerate() {
	// 			summed_errors[idx] += error;
	// 		}
	// 	}

	// 	dbg!(summed_errors);

	// 	assert!(false);
	// }
}
