use num_traits::FromPrimitive;
use std::ops::{AddAssign, Div, SubAssign};

pub trait MovingAverage<Divisor, Sample>
where
    Sample: Copy + PartialOrd + AddAssign + SubAssign + Div<Divisor, Output = Sample>,
    Divisor: FromPrimitive,
{
    fn add_sample(&mut self, new_sample: Sample);
    fn get_num_samples(&self) -> usize;
    fn get_average_sample(&self) -> Sample;
    fn get_most_recent_sample(&self) -> Option<Sample>;
    fn get_samples(&mut self) -> &[Sample];
}
