use support::num::Fibonacci;
use num::bigint::BigUint;
use num::FromPrimitive;
use num::pow;

#[allow(dead_code)]
pub fn get_answer() -> i32 {
	let min = pow(BigUint::from_u64(10).unwrap(), 999);
	let answer = Fibonacci::iter()
		.enumerate()
		.skip_while(|&(_, ref v)| v < &min)
		.nth(0)
		.unwrap();
	answer.0 as i32 + 1
}
