use std::collections::HashMap;
use std::hash::Hash;

// Old
mod dyn_bit_field;
use dyn_bit_field::DynamicBitField;
mod bit_field;

//mod bit_field_helper;

pub use bit_field::BitField;

/// This is the main star of this crate.
/// It is an efficient model of a vector of elements,
/// where each element is just a set of tags.
///
/// This datatype is intended to handle requests for a huge set
/// of elements whose tags fulfill a requirement, i.e. "all elements
/// with the tag 'fruit' but not with the tag 'apple'.
///
/// It is expected that the elements share a lot of tags, i.e.
/// there are a lot fewer tags than elements.
///
/// It is not optimized for simply iterating over the tags
/// of each element, hence it is not recommended to do such
/// a thing with this datatype too much.
#[derive(PartialEq, Eq)]
pub struct BitVec<T, F = u32>
		where T: Eq + Hash, F: BitField {
	tag_fields: HashMap<T, DynamicBitField<F>>,
	len: usize,
}

impl<T: Eq + Hash, F: BitField> BitVec<T, F> {
	// I don't think this needs an example?
	/// Creates an empty, new bit vector.
	pub fn new() -> BitVec<T, F> {
		BitVec {
			tag_fields: HashMap::new(),
			len: 0,
		}
	}

	/// The number of elements in the BitVec
	pub fn len(&self) -> usize { self.len }

	/// Pushes a new element onto the bitvec,
	/// where the new element is defined
	/// as an iterator of tags(borrows of tags specifically)
	pub fn push<'a, I, Q>(&mut self, tags: I) 
		where I: IntoIterator<Item = &'a Q>,
				Q: std::borrow::Borrow<T> + 'a {

	}

	/// Iterates over each tag of an element(an element is considered
	/// to _be_ its tags).
	/// Will panic! if the index is out of bounds.
	pub fn get_element<'a>(&'a self, index: usize) 
			-> impl Iterator<Item = &'a T> + 'a {
		unimplemented!();
		self.tag_fields.keys()
	}
}

#[cfg(test)]
mod tests {
}
