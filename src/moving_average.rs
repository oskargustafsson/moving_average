/// The simple moving average is defined as `sum(window(samples, N)) / length(window(samples, N))`.
/// Here `samples` is a possibly infinite series of samples. The `window` function extracts the last
/// `N` samples.
///
/// For the purposes of this library, there is no point in keeping samples outside the sample window
/// around, so they are discarded when newer samples push them out of the window.
///
/// Terminology:
///  - Sample: A data point.
///  - Sample window: The subset of all samples used for average calculations.
pub trait MovingAverage<Divisor, Sample> {
	/// Adds a sample to the series of samples. If the sample window is full, this will cause the
	/// oldest sample to be dropped, i.e. no longer contribute to the average.
	fn add_sample(&mut self, new_sample: Sample);

	/// Returns the simple moving average value of all the samples in the sample window.
	fn get_average_sample(&self) -> Sample;

	/// Returns the most recently added sample.
	fn get_most_recent_sample(&self) -> Option<Sample>;

	/// Returns a reference to a slice, containing all samples in the sample window.
	fn get_samples(&mut self) -> &[Sample];

	/// Returns the total number of samples in the sample window.
	fn get_num_samples(&self) -> usize;
}
