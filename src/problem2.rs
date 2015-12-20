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
