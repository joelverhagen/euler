use support::sieve::SieveOfEratosthenes;

#[allow(dead_code)]
pub fn get_answer() -> i32 {
	SieveOfEratosthenes::start().take(10001).last().unwrap()
}
