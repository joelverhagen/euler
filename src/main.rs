mod support;
mod problems;

extern crate num;
extern crate chrono;

use problems::problem21::get_answer;

fn main() {
    println!("Problem: {}", get_answer());
}
