use num::bigint::{BigInt, BigUint};
use num::FromPrimitive;
use std::mem::replace;

pub fn factorial(n: i32) -> BigInt {
    (2..n+1).fold(BigInt::from_i32(1).unwrap(), |p, v| p * BigInt::from_i32(v).unwrap())
}

pub fn binomial_coefficient(n: i32, k: i32) -> BigInt {
    factorial(n) / (factorial(n - k) * factorial(k))
}

pub struct Fibonacci {
	previous: BigUint,
	next: BigUint,
}

impl Fibonacci {
	pub fn iter() -> Fibonacci {
		Fibonacci { previous: BigUint::from_u8(0).unwrap(), next: BigUint::from_u8(1).unwrap() }
	}
}

impl Iterator for Fibonacci {
	type Item = BigUint;

	fn next(&mut self) -> Option<BigUint> {
		let new_next = &self.previous + &self.next;
		let next = replace(&mut self.next, new_next);
		self.previous = next.clone();
		Some(next)
	}
}
