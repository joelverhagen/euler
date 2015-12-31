use num::pow;
use support::digits::HasDigits;

fn get_max(p: usize) -> u64 {
	let mut digits = 2;
	let mut max = 0;
	loop {
		let new_max = digits as u64 * pow(9u64, p);
		if new_max.iter_digits().count() < digits {
			break;
		}

		max = new_max;
		digits += 1;
	}

	max
}

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	let p = 5;
	let mut sum = 0;
	for i in 10..get_max(p) + 1 {
		let maybe_i = i.iter_digits().fold(0, |s, d| s + pow(d as u64, p));
		if i == maybe_i {
			sum += i;
		}
	}

	sum
}
