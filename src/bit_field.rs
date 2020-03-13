/// A trait for a type that can
/// work as a bitfield.
pub trait BitField {
	/// Creates a bitfield with no bits
	/// set
	fn empty() -> Self;

	/// Sets a bit. Assumes that n is less than
	/// n_bits
	fn set_bit(&mut self, n: usize, state: bool);

	/// Gets a bit. Assumes that n is less than
	/// n_bits
	fn get_bit(&self, n: usize) -> bool;

	/// Returns the number of bits.
	fn n_bits() -> usize;
}

/// Implements the BitField trait for a numeric type
macro_rules! impl_bitfield {
	($t:ty) => {
		impl BitField for $t {
			fn empty() -> Self { 0 }

			fn set_bit(&mut self, n: usize, state: bool) {
				if state {
					*self |= (1 << n);
				}else{
					*self &= !(1 << n);
				}	
			}

			fn get_bit(&self, n: usize) -> bool {
				*self & (1 << n) > 0
			}

			fn n_bits() -> usize { 8 }
		}
	}
}

impl_bitfield!(u8);
impl_bitfield!(u16);
impl_bitfield!(u32);
impl_bitfield!(u64);
impl_bitfield!(u128);

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn get_bits() {
		assert_eq!(0b00101u32.get_bit(0), true);
		assert_eq!(0b00101u32.get_bit(1), false);
		assert_eq!(0b00101u32.get_bit(2), true);
		assert_eq!(0b00101u32.get_bit(3), false);
	}

	#[test]
	fn set_bits() {
		let mut bits = 0b00000u32;
		bits.set_bit(5, true);
		assert_eq!(bits.get_bit(5), true);
		bits.set_bit(5, false);
		assert_eq!(bits.get_bit(5), false);
	}
}
