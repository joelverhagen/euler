use support::num::binomial_coefficient;
use num::ToPrimitive;

#[allow(dead_code)]
pub fn get_answer() -> u64 {
    binomial_coefficient(40, 20).to_u64().unwrap()
}
