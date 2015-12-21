mod support;
mod problems;
extern crate num;

use problems::problem16::get_answer;

fn main() {
    println!("Problem: {}", get_answer());
}
