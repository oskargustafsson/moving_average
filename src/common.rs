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
