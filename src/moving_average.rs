pub trait MovingAverage<Divisor, Sample> {
	fn add_sample(&mut self, new_sample: Sample);
	fn get_average_sample(&self) -> Sample;
	fn get_most_recent_sample(&self) -> Option<Sample>;
	fn get_samples(&mut self) -> &[Sample];
	fn get_num_samples(&self) -> usize;
}
