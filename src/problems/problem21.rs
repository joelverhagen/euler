use support::factors::ProperDivisors;

fn d(n: u64) -> u64 {
	ProperDivisors::iter(n).fold(0, |s, v| s + v) - n
}

fn is_amicable(n: u64) -> bool {
	let other = d(n);
	other != n && n == d(other) 
}

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	(1..10000).filter(|&v| is_amicable(v)).fold(0, |s, v| s + v)
}
