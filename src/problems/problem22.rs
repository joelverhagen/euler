use support::files::read_file;

fn get_sorted_names() -> Vec<String> {
	let file_content = read_file("../data/p022_names.txt").unwrap();
	let mut names: Vec<String> = Vec::new();
	for name in file_content.split(',') {
		names.push(name[1..name.len() - 1].to_string());
	}

	names.sort();

	names
}

fn get_letter_sum(input: &str) -> u32 {
	let mut sum: u32 = 0;
	for c in input.chars().filter(|&c| c.is_alphabetic()) {
		sum += c.to_digit(36).unwrap() - 9;
	}

	sum
}

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	let names = get_sorted_names();
	let mut sum = 0;
	for i in 0..names.len() {
		sum += (i as u64 + 1) * get_letter_sum(&names[i]) as u64;
	}

	sum
}
