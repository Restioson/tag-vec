use crate::bit_field::BitField;

// TODO: Make the SmallBitField generic
/// A dynamically sized bit field.
#[derive(PartialEq, Eq)]
pub(crate) struct DynamicBitField<T: BitField> {
	data: Vec<T>,
	len: usize,
}

impl<T: BitField> DynamicBitField<T> {
	/// Creates a new DynamicBitField
	// Unused for now
	// pub(crate) fn new() -> DynamicBitField<T> {
	// 	DynamicBitField {
	// 		data: Vec::new(),
	// 		len: 0,
	// 	}
	// }

	pub(crate) fn data<'a>(&'a self) -> &'a [T] {
		&self.data[..]
	}

	/// Creates a DynamicBitField with n_bits bits set to false
	pub(crate) fn with_false(n_bits: usize) -> DynamicBitField<T> {
		let mut data = Vec::with_capacity(1 + (n_bits / T::n_bits()));
		for _ in 0..(1 + (n_bits / T::n_bits())) {
			data.push(T::empty());
		}

		DynamicBitField {
			data, len: n_bits
		}
	}

	/// Returns the length in bits
	/// of the DynamicBitField
	pub(crate) fn len(&self) -> usize {
		self.len
	}

	/// Pushes a bit onto the DynamicBitField
	/// Panics if the size overflows usize
	pub(crate) fn push(&mut self, value: bool) {
		// Make sure it doesn't overflow
		assert!(self.len < std::usize::MAX);

		let (data_index, bit_index) = get_indices::<T>(self.len);
		self.len += 1;

		if self.data.len() <= data_index {
			self.data.push(T::empty());
		}

		self.data[data_index].set_bit(bit_index, value);
	}

	// pub(crate) fn set_unchecked(&mut self, index: usize, value: bool) {
	// 	let (data_index, bit_index) = get_indices::<T>(index);
	// 	self.data[data_index].set_bit(bit_index, value);
	// }

	/// Returns a value at the index.
	/// Panics if the index is out of bounds 
	pub(crate) fn get_unchecked(&self, index: usize) -> bool {
		let (data_index, bit_index) = get_indices::<T>(index);
		self.data[data_index].get_bit(bit_index)
	}
}

/// Returns the (data index, local bit index) pair for
/// a bit index. The name is really bad, but it's private,
/// so I think it's fine
fn get_indices<T: BitField>(bit_index: usize) -> (usize, usize) {
	let n_bits_in_elem = T::n_bits();
	(bit_index / n_bits_in_elem, bit_index % n_bits_in_elem)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn field() {
		let mut field = DynamicBitField::<u32>::with_false(0);
		field.push(true);
		field.push(false);
		field.push(true);

		assert_eq!(field.len(), 3);
		assert_eq!(field.get_unchecked(0), true);
		assert_eq!(field.get_unchecked(1), false);

		for _ in 3..101 {
			field.push(true);
		}

		assert_eq!(field.get_unchecked(100), true);
	}
}
