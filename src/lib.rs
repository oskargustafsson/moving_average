/*!
This crate provides several algorithms for calculating
[simple moving average](https://en.wikipedia.org/wiki/Moving_average#Simple_moving_averages).

All implementations implement the MovingAverage trait, which provides a unified iterface.

The implementations have different pros and cons.

| Implementation         | Add sample | Get average | Comment |
|------------------------|------------|-------------|---------|
| NoSumMovingAverage     | O(1)       | O(n)        |  |
| SingleSumMovingAverage | O(1)       | O(1)        | May accumulate floating point rounding errors. |
| SumTreeMovingAverage   | O(log(n))  | O(1)        |  |

`n` in the above chart refers to the sample size of the moving average window.

*/
mod moving_average;
mod no_sum_moving_average;
mod single_sum_moving_average;
mod sum_tree;
mod sum_tree_moving_average;

pub use moving_average::MovingAverage;
pub use no_sum_moving_average::NoSumMovingAverage;
pub use single_sum_moving_average::SingleSumMovingAverage;
pub use sum_tree_moving_average::SumTreeMovingAverage;

#[cfg(test)]
mod tests {
    use crate::{MovingAverage, NoSumMovingAverage, SingleSumMovingAverage, SumTreeMovingAverage};
    use nalgebra::Vector2;
    use std::time::Duration;

    macro_rules! get_ma_impls {
		(
			$divisor_type:ty, $max_num_samples:expr, $ctor:ident $(, $zero:expr)?
		) => {{
			let ma_impls: [Box<dyn MovingAverage<$divisor_type, _>>; 3] = [
				Box::new(SingleSumMovingAverage::$ctor($($zero ,)? $max_num_samples)),
				Box::new(SumTreeMovingAverage::$ctor($($zero ,)? $max_num_samples)),
				Box::new(NoSumMovingAverage::$ctor($($zero ,)? $max_num_samples)),
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
    fn vector2_f32_samples() {
        for ma in &mut get_ma_impls!(f32, 3, new) {
            assert_eq!(ma.get_average_sample(), Vector2::new(0.0, 0.0));

            ma.add_sample(Vector2::<f32>::new(4.0, 8.0));
            assert_eq!(ma.get_average_sample(), Vector2::new(4.0, 8.0));

            ma.add_sample(Vector2::<f32>::new(6.0, 0.0));
            assert_eq!(ma.get_average_sample(), Vector2::new(5.0, 4.0));

            ma.add_sample(Vector2::<f32>::new(2.0, 10.0));
            assert_eq!(ma.get_average_sample(), Vector2::new(4.0, 6.0));

            ma.add_sample(Vector2::<f32>::new(-17.0, 20.0));
            assert_eq!(ma.get_average_sample(), Vector2::new(-3.0, 10.0));

            ma.add_sample(Vector2::<f32>::new(0.0, -21.0));
            assert_eq!(ma.get_average_sample(), Vector2::new(-5.0, 3.0));
        }
    }

    #[test]
    fn duration_samples() {
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
