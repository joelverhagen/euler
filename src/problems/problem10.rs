use support::sieve::SieveOfEratosthenes;

#[allow(dead_code)]
pub fn get_answer() -> u64 {
    SieveOfEratosthenes::get_primes_up_to(2000000).iter().fold(0, |s, p| s + p)
}
