use support::digits::HasDigits;

fn digit_to_word(n: u8) -> Option<String> {
	match n {
		0 => Some("zero".to_string()),
		1 => Some("one".to_string()),
		2 => Some("two".to_string()),
		3 => Some("three".to_string()),
		4 => Some("four".to_string()),
		5 => Some("five".to_string()),
		6 => Some("six".to_string()),
		7 => Some("seven".to_string()),
		8 => Some("eight".to_string()),
		9 => Some("nine".to_string()),
		_ => None,
	}
}

fn ten_to_word(n: u8) -> Option<String> {
	match n {
		1 => Some("ten".to_string()),
		2 => Some("twenty".to_string()),
		3 => Some("thirty".to_string()),
		4 => Some("forty".to_string()),
		5 => Some("fifty".to_string()),
		6 => Some("sixty".to_string()),
		7 => Some("seventy".to_string()),
		8 => Some("eighty".to_string()),
		9 => Some("ninety".to_string()),
		_ => None,
	}
}

fn place_to_word(n: u8) -> Option<String> {
	match n {
		3 => Some("thousand".to_string()),
		6 => Some("million".to_string()),
		9 => Some("billion".to_string()),
		12 => Some("trillion".to_string()),
		_ => None,
	}
}

fn ten_and_unit_to_words(t: u8, u: u8) -> Option<Vec<String>> {
	match (t, u) {
		(0, _) => match digit_to_word(u) {
			Some(w) => Some(vec![w]),
			None => None,
		},
		(1, _) => match u {
			0 => Some(vec!["ten".to_string()]),
			1 => Some(vec!["eleven".to_string()]),
			2 => Some(vec!["twelve".to_string()]),
			3 => Some(vec!["thirteen".to_string()]),
			4 => Some(vec!["fourteen".to_string()]),
			5 => Some(vec!["fifteen".to_string()]),
			6 => Some(vec!["sixteen".to_string()]),
			7 => Some(vec!["seventeen".to_string()]),
			8 => Some(vec!["eighteen".to_string()]),
			9 => Some(vec!["nineteen".to_string()]),
			_ => None,
		},
		(2 ... 9, 0) => match ten_to_word(t) {
			Some(tw) => Some(vec![tw]),
			None => None,
		},
		(2 ... 9, _) => match (ten_to_word(t), digit_to_word(u)) {
			(Some(tw), Some(uw)) => Some(vec![tw + &"-" + &uw]),
			_ => None,
		},
		_ => None,
	}
}

fn get_digits(n: u64) -> Vec<u8> {
	let mut digits: Vec<u8> = n.iter_digits().collect();
	while digits.len() % 3 != 0 {
		digits.push(0);
	}

	digits
}

fn to_words(n: u64) -> String {
	let mut pieces: Vec<String> = Vec::new();
	
	if n == 0 {
		pieces.push(digit_to_word(0).unwrap());
	} else {
		let digits = get_digits(n);
		for g in (0..digits.len() / 3).rev() {
			let u = digits[((3 * g) + 0) as usize];
			let t = digits[((3 * g) + 1) as usize];
			let h = digits[((3 * g) + 2) as usize];

			if h > 0 {
				pieces.push(digit_to_word(h).unwrap());
				pieces.push("hundred".to_string());
			}

			if t > 0 || u > 0 {
				if h > 0 {
					pieces.push("and".to_string());
				}

				let ten_and_unit = ten_and_unit_to_words(t, u);
				if ten_and_unit.is_some() {
					for p in ten_and_unit.unwrap() {
						pieces.push(p);
					}
				}
			}

			if g > 0 {
				pieces.push(place_to_word((3 * g) as u8).unwrap());
			}
		}
	}

	pieces.join(" ")
}

#[allow(dead_code)]
pub fn get_answer() -> i32 {
	let mut total = 0;
	for i in 1..1001 {
		let words = to_words(i);
		let count = words.chars().filter(|&c| c.is_alphabetic()).count() as i32;
		// print!("{} = {}, {} + {} = ", i, words, total, count);
		total += count;
		// println!("{}", total);
	}
	
	total
}
