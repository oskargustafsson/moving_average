pub struct RingBuffer<Item, const CAPACITY: usize> {
	items: [Item; CAPACITY],
	front_idx: usize,
	num_items: usize,
}

impl<Item: Copy, const CAPACITY: usize> RingBuffer<Item, CAPACITY> {
	pub fn new(zero: Item) -> Self {
		Self {
			items: [zero; CAPACITY],
			front_idx: 0, // Index of the next available slot
			num_items: 0,
		}
	}

	pub fn shift(&mut self, item: Item) -> Option<Item> {
		let popped_item = if self.len() == CAPACITY {
			self.pop_back()
		} else {
			None
		};
		self.push_front(item);
		popped_item
	}

	pub fn push_front(&mut self, item: Item) {
		self.items[self.front_idx] = item;
		self.front_idx = wrapping_add::<CAPACITY>(self.front_idx, 1);
		self.num_items = CAPACITY.min(self.num_items + 1);
	}

	pub fn pop_back(&mut self) -> Option<Item> {
		if 0 < self.num_items {
			let num_items = self.num_items;
			self.num_items -= 1;
			Some(self.items[wrapping_sub::<CAPACITY>(self.front_idx, num_items)])
		} else {
			None
		}
	}

	pub fn front(&self) -> Option<&Item> {
		if 0 < self.num_items {
			Some(&self.items[wrapping_sub::<CAPACITY>(self.front_idx, 1)])
		} else {
			None
		}
	}

	pub fn len(&self) -> usize {
		self.num_items
	}

	pub fn iter(&self) -> Iter<'_, Item, CAPACITY> {
		Iter {
			items: &self.items,
			cursor_idx: wrapping_sub::<CAPACITY>(self.front_idx, self.num_items),
			num_items_left: self.num_items,
		}
	}
}

fn wrapping_add<const MAX_VAL: usize>(lhs: usize, rhs: usize) -> usize {
	(lhs + rhs) % MAX_VAL
}

fn wrapping_sub<const MAX_VAL: usize>(lhs: usize, rhs: usize) -> usize {
	debug_assert!(rhs <= MAX_VAL);
	if lhs < rhs {
		(MAX_VAL - rhs) + lhs
	} else {
		lhs - rhs
	}
}

pub struct Iter<'a, Item: 'a, const CAPACITY: usize> {
	items: &'a [Item],
	cursor_idx: usize,
	num_items_left: usize,
}

impl<'a, Item, const CAPACITY: usize> Iterator for Iter<'a, Item, CAPACITY> {
	type Item = &'a Item;

	fn next(&mut self) -> Option<Self::Item> {
		if self.num_items_left == 0 {
			return None;
		}

		self.num_items_left -= 1;

		let cursor_idx = self.cursor_idx;
		self.cursor_idx = wrapping_add::<CAPACITY>(self.cursor_idx, 1);

		Some(&self.items[cursor_idx])
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn assert_rb_state(rb: &RingBuffer<u32, 3>, items: &[u32]) {
		assert_eq!(rb.len(), items.len());
		assert_eq!(rb.front(), items.get(items.len().wrapping_sub(1)));
		assert_eq!(
			rb.iter().collect::<Vec<&u32>>(),
			items.iter().collect::<Vec<&u32>>()
		);
	}

	#[test]
	fn basics() {
		let mut rb: RingBuffer<u32, 3> = RingBuffer::new(0);

		assert_eq!(rb.pop_back(), None);
		assert_rb_state(&rb, &[]);

		rb.push_front(1);
		assert_rb_state(&rb, &[1]);

		assert_eq!(rb.pop_back(), Some(1));
		assert_rb_state(&rb, &[]);

		rb.push_front(1);
		assert_eq!(rb.shift(2), None);
		assert_rb_state(&rb, &[1, 2]);

		assert_eq!(rb.shift(3), None);
		assert_rb_state(&rb, &[1, 2, 3]);

		assert_eq!(rb.shift(4), Some(1));
		assert_rb_state(&rb, &[2, 3, 4]);

		rb.push_front(5);
		assert_rb_state(&rb, &[3, 4, 5]);

		assert_eq!(rb.pop_back(), Some(3));
		assert_rb_state(&rb, &[4, 5]);

		assert_eq!(rb.pop_back(), Some(4));
		assert_rb_state(&rb, &[5]);

		assert_eq!(rb.shift(6), None);
		assert_rb_state(&rb, &[5, 6]);

		assert_eq!(rb.pop_back(), Some(5));
		assert_rb_state(&rb, &[6]);

		assert_eq!(rb.pop_back(), Some(6));
		assert_rb_state(&rb, &[]);

		assert_eq!(rb.pop_back(), None);
		assert_rb_state(&rb, &[]);
	}
}
