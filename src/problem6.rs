use std::slice::Iter;

fn sum_of_square(v: &Vec<i32>) -> i32 {
	v.iter().map(|&i| i*i).fold(0, |s, i| s + i)
}

fn square_of_sum(v: &Vec<i32>) -> i32 {
	let sum = v.iter().fold(0, |s, i| s + i);
	sum * sum
}


pub fn get_answer() -> i32 {
	let v: Vec<i32> = (1..101).collect();
	let sum = sum_of_square(&v);
	let sqr = square_of_sum(&v);
	sqr - sum
}