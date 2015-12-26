use support::num::Fibonacci;
use num::bigint::BigUint;
use num::ToPrimitive;
use num::FromPrimitive;
use num::Zero;
use num::Integer;

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	Fibonacci::iter()
		.take_while(|x| *x <= BigUint::from_u64(4000000).unwrap())
		.filter(|x| x.is_even())
		.fold(BigUint::zero(), |s, i| s + i)
		.to_u64()
		.unwrap()
}
