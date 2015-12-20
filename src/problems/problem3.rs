struct PrimeFactors {
	current: u64,
	of: u64,
}

impl Iterator for PrimeFactors {
	type Item = u64;
	fn next(&mut self) -> Option<u64> {
		let mut factor = None;
		while factor.is_none() && self.of > 1 {
			self.current += 1;

			if self.of % self.current == 0 {
				self.of /= self.current;
				factor = Some(self.current);
			}
		}

		factor
	}
}

fn factor(of: u64) -> PrimeFactors {
	PrimeFactors { current: 1, of: of }
}

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	factor(600851475143).max().unwrap()
}
