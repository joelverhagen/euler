use num::pow;
use num::bigint::BigInt;
use num::FromPrimitive;
use support::numbers::HasDigits;

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	pow(BigInt::from_u64(2).unwrap(), 1000)
		.iter_digits()
		.map(|d| d as u64)
		.fold(0, |s, v| s + v)
}
