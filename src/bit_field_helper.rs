use crate::BitField;

/// Returns the index of a byte in terms of BitField index and local bit index 
pub(crate) fn bitfield_index<T: BitField>(bit_index: usize) -> (usize, usize) {
	let n_bits = T::n_bits();
	(bit_index / n_bits, bit_index % n_bits)
}

/// Add a bit to a dynamic bit field. Make sure that the field you pass in
/// already can fit field_len bits, since this is assumed.
pub(crate) fn push_to_bitfield<T: BitField>(field: &mut Vec<T>, field_len: &mut usize, value: bool) {
	assert!(*field_len < std::usize::MAX);

	let (bit_field_index, local_bit_index) = bitfield_index::<T>(*field_len);

	*field_len += 1;
	// Make sure the field can fit the necessary data
	if field.len() <= bit_field_index {
		field.push(T::empty());
	}

	// Set the local bit
	field[bit_field_index].set_bit(local_bit_index, value);
}

/// Assumes the index is smaller than the number of bits in the field.
/// This will never modify the field
pub(crate) fn get_bit<T: BitField>(field: &[T], index: usize) -> bool {
	let (bit_field_index, local_bit_index) = bitfield_index::<T>(index);
	field[bit_field_index].get_bit(local_bit_index)
}

pub(crate) fn new_empty_field<T: BitField>(n_bits: usize) -> Vec<T> {
	let n_members = 
}
