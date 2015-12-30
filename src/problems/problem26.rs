use std::collections::HashMap;
use std::fmt::{Display, Result, Formatter};

struct Decimal {
	integer: u64,
	fractional: FractionalPart,
}

struct FractionalPart {
	terminating: Vec<u8>,
	repeating: Vec<u8>,
}

impl Display for Decimal {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}.", self.integer).unwrap();
		if self.fractional.terminating.len() > 0 || self.fractional.repeating.len() > 0 {
			for i in &self.fractional.terminating {
				write!(f, "{}", i).unwrap();
			}

			for i in &self.fractional.repeating {
				write!(f, "{}", i).unwrap();
			}

			if self.fractional.repeating.len() > 0 {
				write!(f, "...").unwrap();
			}
		} else {
			write!(f, "0").unwrap();
		}

		write!(f, "")
    }
}

impl Decimal {
	fn from_fraction(numerator: u64, denominator: u64) -> Decimal {
		let integer = numerator / denominator;
		
		let mut found: HashMap<u64, usize> = HashMap::new();
		let mut remainder = numerator % denominator;
		let mut position = 0;
		let mut terminating = Vec::new();
		let repeating;
		loop {
			if remainder == 0 {
				repeating = Vec::new();
				break;
			}

			let entry_position = *found.entry(remainder).or_insert(position);
			if entry_position != position {
				repeating = terminating[entry_position..].to_vec();
				terminating.truncate(entry_position);
				break;
			}

			remainder *= 10;
			terminating.push((remainder / denominator) as u8);
			remainder %= denominator;
			position += 1;
		}

		Decimal {
			integer: integer,
			fractional: FractionalPart {
				terminating: terminating,
				repeating: repeating,
			}
		}
	}
}

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	let mut max_len = 0;
	let mut max_d = 0;
	for d in 2..1000 {
		let len = Decimal::from_fraction(1, d).fractional.repeating.len();
		if len > max_len {
			max_d = d;
			max_len = len;
		}
	}

	max_d
}
