mod support;
mod problems;

extern crate num;
extern crate chrono;
extern crate permutohedron;

use problems::problem25::get_answer;

fn main() {
    println!("Problem: {}", get_answer());
}
