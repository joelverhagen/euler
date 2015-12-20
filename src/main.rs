mod problem1 {
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

	pub fn get_answer() -> i32 {
		get_multiples(vec![3, 5]).take_while(|&x| x < 1000).fold(0, |s, i| s + i)
	}
}

mod problem2 {
	struct Fibonacci {
		previous: i32,
		current: i32,
	}

	impl Iterator for Fibonacci {
		type Item = i32;

		fn next(&mut self) -> Option<i32> {
			let next = self.previous + self.current;
			self.previous = self.current;
			self.current = next;
			Some(next)
		}
	}

	fn get_fibonacci() -> Fibonacci {
		Fibonacci { previous: 0, current: 1 }
	}

	pub fn get_answer() -> i32 {
		get_fibonacci().take_while(|&x| x <= 4000000).filter(|&x| x % 2 == 0).fold(0, |s, i| s + i)
	}
}

fn main() {
    println!("Problem 1: {}", problem1::get_answer());
    println!("Problem 2: {}", problem2::get_answer());
}
