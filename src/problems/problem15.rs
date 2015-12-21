use num::bigint::BigInt;
use num::FromPrimitive;
use num::ToPrimitive;

fn factorial(n: i32) -> BigInt {
    (2..n+1).fold(BigInt::from_i32(1).unwrap(), |p, v| p * BigInt::from_i32(v).unwrap())
}

fn binomial_coefficient(n: i32, k: i32) -> BigInt {
    factorial(n) / (factorial(n - k) * factorial(k))
}

#[allow(dead_code)]
pub fn get_answer() -> u64 {
    binomial_coefficient(40, 20).to_u64().unwrap()
}
