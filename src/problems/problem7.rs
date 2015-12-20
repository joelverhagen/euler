use support::sieve::SieveOfEratosthenes;

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	SieveOfEratosthenes::iter().take(10001).last().unwrap()
}
