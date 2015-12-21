use num::pow;
use num::bigint::BigInt;
use num::FromPrimitive;

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	pow(BigInt::from_u64(2).unwrap(), 1000)
		.to_str_radix(10)
		.chars()
		.map(|c| c.to_digit(10).unwrap() as u64)
		.fold(0, |s, v| s + v)
}
