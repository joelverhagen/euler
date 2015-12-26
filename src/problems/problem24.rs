use permutohedron::LexicalPermutation;

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	let mut digits = (0..10).collect::<Vec<_>>();
	let mut next = true;
	let mut i = 1;
	while next && i < 1000000 {
		next = digits.next_permutation();
		i += 1;
	}

	digits
	 	.iter()
	 	.rev()
	 	.enumerate()
	 	.map(|(i, v)| v * 10u64.pow(i as u32))
	 	.fold(0, |s, v| s + v)
}
