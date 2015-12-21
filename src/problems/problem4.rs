use support::digits::HasDigits;

fn is_palindrome(i: i32) -> bool {
	let digits: Vec<u8> = i.iter_digits().collect();
	let len = digits.len();
	for i in 0..len / 2 {
		if digits[i] != digits[len - (i + 1)] {
			return false;
		}
	}

	true
}

#[allow(dead_code)]
pub fn get_answer() -> i32 {
	let mut largest = 0;
	for x in 100..1000 {
		for y in x..1000 {
			let product = x * y;
			if is_palindrome(product) && product > largest {
				largest = product;
			}
		}
	}

	largest
}
