use support::factors::ProperDivisors;
use std::collections::HashSet;

fn sum_of_proper_divisors(n: u64) -> u64 {
	ProperDivisors::iter(n).fold(0, |s, v| s + v)
}

fn is_abundant(n: u64) -> bool {
	sum_of_proper_divisors(n) > n
}

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	// find all abundant numbers <= 28123
	let mut abundant_vec = Vec::new();
	let mut abundant_set = HashSet::new();
	for i in 1..28124 {
		if is_abundant(i) {
			abundant_vec.push(i);
			abundant_set.insert(i);
		}
	}

	// find all numbers that are not the sum of two abundant numbers
	let mut sum = 0;
	for i in 1..28124 {
		let mut found = false;
		for abundant in &abundant_vec {
			if *abundant >= i {
				break;
			}

			let other = i - abundant;
			if abundant_set.contains(&other) {
				found = true;
				break;
			}
		}

		if !found {
			sum += i;
		}
	}

	sum
}
