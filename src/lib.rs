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
pub struct TagVec<T, F = u32>
		where T: Eq + Hash + Clone, F: BitField {
	tag_fields: HashMap<T, DynamicBitField<F>>,
	len: usize,
}

impl<T: Eq + Hash + Clone, F: BitField> TagVec<T, F> {
	// I don't think this needs an example?
	/// Creates an empty, new bit vector.
	pub fn new() -> TagVec<T, F> {
		TagVec {
			tag_fields: HashMap::new(),
			len: 0,
		}
	}

	/// The number of elements in the TagVec
	pub fn len(&self) -> usize { self.len }

	/// Pushes a new element onto the bitvec,
	/// where the new element is defined
	/// as an iterator of tags(borrows of tags specifically)
	/// 
	/// And OMG the generics on this function are
	/// crazy
	pub fn push<'a, I, Q>(&mut self, tags: I) 
		where I: IntoIterator<Item = &'a Q>,
				Q: ?Sized + Hash + Eq + 'a,
				T: From<&'a Q> + std::borrow::Borrow<Q> {
		// Vec doesn't allocate when created, and
		// we will rarely see unknown tags come forth,
		// so this won't be a performance hog
		let mut skipped_tags: Vec<T> = Vec::new();	

		// Add tags to existing tag fields
		for tag in tags {
			match self.tag_fields.get_mut(tag) {
				Some(field) => field.push(true),
				None => skipped_tags.push(tag.into()),
			}
		}

		// Push false to any tag fields that this element didn't contain
		for tag_field in self.tag_fields.values_mut() {
			if tag_field.len() < self.len + 1 {
				tag_field.push(false);
			}
		}

		// Create new tag fields for tags that appeared just now
		// This shouldn't run too often since there are fewer tags than values hopefully
		for skipped_tag in skipped_tags {
			let mut new_field = DynamicBitField::with_false(self.len());
			new_field.push(true); // This is the first element to have the tag
			self.tag_fields.insert(skipped_tag, new_field);
		}

		self.len += 1;
	}

	/// Iterates over each tag of an element(an element is considered
	/// to _be_ its tags). The iterator is unordered, so be careful.
	/// Will panic if the index is out of bounds.
	///
	/// Examples:
	/// ```
	/// use tag_vec::TagVec;
	/// // It is good to give the type of the key to
	/// // the type, as it may be difficult for the compiler
	/// // to infer it
	/// let mut tag_vec: TagVec<String> = TagVec::new();
	/// tag_vec.push(vec!["hello", "world"]);
	///
	/// // We should find a "hello" tag but not a "hi" tag
	/// // in the iterator
	/// let tags = tag_vec.iter_element(0);
	/// assert!(tags.clone().any(|v| *v == "hello"));
	/// assert!(!tags.clone().any(|v| *v == "hi"));
	/// ```
	pub fn iter_element<'a>(&'a self, index: usize) -> IterElement<'a, T, F>
	{
		assert!(index < self.len(), "Cannot iter over an element out of bounds");

		IterElement {
			fields: self.tag_fields.iter(),
			element_id: index
		}
	}
}

/// Iterates over every tag over an element.
/// See ``TagVec::iter_element`` for more
/// information.
#[derive(Clone)]
pub struct IterElement<'a, T, F>
		where T: Eq + Hash + Clone, F: BitField {
	fields: std::collections::hash_map::Iter<'a, T, DynamicBitField<F>>,
	element_id: usize,
}

impl<T: Eq + Hash + Clone, F: BitField> std::iter::FusedIterator for IterElement<'_, T, F> {}

impl<'a, T: Eq + Hash + Clone, F: BitField> Iterator for IterElement<'a, T, F> {
	type Item = &'a T;

	fn next(&mut self) -> Option<&'a T> {
		// Try to find the next field that contains this element.
		// Once you find one, return it. 
		while let Some((key, field)) = self.fields.next() {
			if field.get_unchecked(self.element_id) {
				return Some(key);
			}
		}

		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn pushing() {
		let mut tags = TagVec::<String>::new();
		assert_eq!(tags.tag_fields.len(), 0);
		tags.push(vec!["hello", "sir"]);
		tags.push(vec!["other", "sir"]);

		// Testing implementation detail thing, not part of
		// the API
		assert_eq!(tags.tag_fields.len(), 3);

		let tag_vec: Vec<_> = tags.iter_element(0).collect();
		assert_eq!(tag_vec.len(), 2);
		assert!(tag_vec.iter().any(|v| *v == "hello"));
		assert!(tag_vec.iter().any(|v| *v == "sir"));
	}
}
