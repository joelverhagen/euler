use std::collections::HashSet;

fn get_smallest_divisible_by(values: Vec<i32>) -> i32 {
	// remove duplicates
	let mut values_set = HashSet::new();
	for v in values {
		values_set.insert(v);
	}

	let mut unique: Vec<i32> = values_set.iter().map(|&v| v).collect();
	unique.sort();

	// remove factors
	let mut reduced: Vec<i32> = Vec::new();
	for i in 0..unique.len() {
		let mut reduceable = false;
		for j in i+1..unique.len() {
			if unique[j] % unique[i] == 0 {
				reduceable = true;
				break;
			}
		}

		if !reduceable {
			reduced.push(unique[i]);
		}
	}

	// find
	let mut i = 0;
	let mut found = false;
	while !found {
		i += 1;
		found = reduced.iter().all(|f| i % f == 0);
	}

	i
}

#[allow(dead_code)]
pub fn get_answer() -> i32 {
	get_smallest_divisible_by((1..21).collect())
}