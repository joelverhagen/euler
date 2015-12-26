use num::bigint::BigInt;
use num::FromPrimitive;

pub fn factorial(n: i32) -> BigInt {
    (2..n+1).fold(BigInt::from_i32(1).unwrap(), |p, v| p * BigInt::from_i32(v).unwrap())
}

pub fn binomial_coefficient(n: i32, k: i32) -> BigInt {
    factorial(n) / (factorial(n - k) * factorial(k))
}
