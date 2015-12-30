use support::factors::Factors;
use support::prime::SieveOfEratosthenes;

fn is_prime(n: i32) -> bool {
	Factors::iter(n as u64).count() <= 2
}

fn get_consecutive_primes(a: i32, b: i32) -> Vec<i32> {
	(0..)
		.map(|n| n * n + a * n + b)
		.take_while(|&n| n > 0 && is_prime(n))
		.collect()
}

#[allow(dead_code)]
pub fn get_answer() -> i32 {
	let primes_to_1000 = SieveOfEratosthenes::get_primes_up_to(1000);

	let mut max_a = 0;
	let mut max_b = 0;
	let mut max_primes = vec![1];
	for a in -999..1000 {
		for b in &primes_to_1000 {
			let b = *b as i32;
			let primes = get_consecutive_primes(a, b);
			if primes.len() > max_primes.len() {
				max_a = a;
				max_b = b;
				max_primes = primes;
			}
		}
	}

	max_a * max_b
}
