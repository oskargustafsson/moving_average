use std::any::type_name;

use num_traits::FromPrimitive;

pub fn cast_to_divisor_type<Divisor: FromPrimitive>(divisor: usize) -> Divisor {
	Divisor::from_usize(divisor).unwrap_or_else(|| {
		panic!(
			"Failed to create a divisor of type {} from {}",
			type_name::<Divisor>(),
			divisor
		)
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn cast_to_divisor_type_success() {
		let divisor = cast_to_divisor_type::<u32>(u32::MAX as usize);
		assert_eq!(divisor, u32::MAX);
	}

	#[test]
	#[should_panic]
	fn cast_to_divisor_type_fail() {
		cast_to_divisor_type::<u32>(u32::MAX as usize + 1);
	}
}
