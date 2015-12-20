struct SieveOfEratosthenes {
	next: i32,
	so_far: Vec<i32>,
}

impl Iterator for SieveOfEratosthenes {
	type Item = i32;
	fn next(&mut self) -> Option<i32> {
		let mut next = None;
		while next.is_none() {
			if !self.so_far.iter().any(|&x| self.next % x == 0) {
				self.so_far.push(self.next);
				next = Some(self.next);
			}

			self.next += 1;
		}

		next
	}
}

fn sieve_of_eratosthenes() -> SieveOfEratosthenes {
	SieveOfEratosthenes { next: 2, so_far: Vec::new() }
}

#[allow(dead_code)]
pub fn get_answer() -> i32 {
	sieve_of_eratosthenes().take(10001).last().unwrap()
}
