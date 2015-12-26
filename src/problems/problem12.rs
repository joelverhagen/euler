use support::triangles::Triangles;
use support::factors::Factors;

#[allow(dead_code)]
pub fn get_answer() -> u64 {
    Triangles::iter().skip_while(|&t| Factors::iter(t).count() < 500).nth(0).unwrap()
}
