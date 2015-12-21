pub struct SieveOfEratosthenes {
	next: u64,
	so_far: Vec<u64>,
}

fn get_next_prime(prime: u64, list: &Vec<u64>) -> Option<u64> {
	let option = list.iter().skip_while(|&p| *p <= prime).skip_while(|&p| *p == 0).nth(0);
	if option.is_some() {
		Some(*option.unwrap())
	} else {
		None
	}
}

impl SieveOfEratosthenes {
	pub fn iter() -> SieveOfEratosthenes {
		SieveOfEratosthenes { next: 2, so_far: Vec::new() }
	}

	pub fn get_primes_up_to(n: u64) -> Vec<u64> {
		let mut list: Vec<u64> = (2..n + 1).collect();
		let mut prime = 1;
		loop {
			let prime_option = get_next_prime(prime, &list);
			if prime * prime > n || prime_option.is_none() {
				break;
			}

			prime = prime_option.unwrap();
			let mut index = ((2 * prime) - 2) as usize;
			while index < list.len() {
				list[index] = 0;
				index += prime as usize;
			}
		}

		list.iter().filter(|&p| *p != 0).map(|&p| p).collect()
	}
}

impl Iterator for SieveOfEratosthenes {
	type Item = u64;
	fn next(&mut self) -> Option<u64> {
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
