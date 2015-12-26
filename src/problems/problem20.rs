use support::num::factorial;
use support::digits::HasDigits;

#[allow(dead_code)]
pub fn get_answer() -> i32 {
	factorial(100).iter_digits().map(|d| d as i32).fold(0, |s, v| s + v)
}
