use std::ops::{Rem, Div};
use std::cmp::Eq;
use std::convert::From;
use num::ToPrimitive;

struct IntegerDigits<T> {
	remaining: T
}

impl<T> Iterator for IntegerDigits<T> where T : Rem<Output=T> + Div<Output=T> + Eq + Clone + From<u8> + ToPrimitive {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		if self.remaining == T::from(0) {
			None
		} else {
			let place = self.remaining.clone() % T::from(10);
			let place_digit = T::to_u8(&place).unwrap();

			let remaining = self.remaining.clone() / T::from(10);
			self.remaining = remaining;
			Some(place_digit)
		}		
	}
}

pub trait HasDigits<T> {
	fn iter_digits(&self) -> IntegerDigits<T>;
}

impl<T> HasDigits<T> for T where T : Clone {
	fn iter_digits(&self) -> IntegerDigits<T> {
		IntegerDigits { remaining: self.clone() }
	}
}