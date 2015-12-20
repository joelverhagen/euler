struct Multiples {
	next: i32,
	of: Vec<i32>
}

impl Iterator for Multiples {
	type Item = i32;

	fn next(&mut self) -> Option<i32> {
		let mut next = None;
		while next.is_none() {
			if self.of.iter().any(|o| self.next % o == 0) {
				next = Some(self.next);
			}

			self.next += 1
		}

		next
	}
}

fn get_multiples(of: Vec<i32>) -> Multiples {
	Multiples { next: *of.iter().min().unwrap(), of: of }
}

#[allow(dead_code)]
pub fn get_answer() -> i32 {
	get_multiples(vec![3, 5]).take_while(|&x| x < 1000).fold(0, |s, i| s + i)
}
