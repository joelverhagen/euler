use num::bigint::BigUint;
use num::pow;
use num::FromPrimitive;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	let mut set = HashSet::new();
	for a in 2..101 {
		for b in 2..101 {
			let result = pow(BigUint::from_u64(a).unwrap(), b);
			set.insert(result);
		}
	}

	set.len() as u64
}
