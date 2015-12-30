mod support;
mod problems;

extern crate num;
extern crate chrono;
extern crate permutohedron;

use problems::problem27::get_answer;

fn main() {
    println!("Problem: {}", get_answer());
}
