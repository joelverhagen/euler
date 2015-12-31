fn get_diagonal_sum(grid: &Vec<Vec<u64>>, start: (usize, usize), translation: (i32, i32), length: usize) -> u64 {
	let mut pos = start;
	let mut sum = 0;
	for _ in 0..length {
		sum += grid[pos.0][pos.1];
		pos = ((pos.0 as i32 + translation.0) as usize, (pos.1 as i32 + translation.1) as usize);
	}

	sum
}

fn make_spiral(size: usize) -> Vec<Vec<u64>> {
	let mut spiral = Vec::new();
	for _ in 0..size {
		let mut row = Vec::new();
		for _ in 0..size {
			row.push(0);
		}

		spiral.push(row);
	}

	let mut pos = (size / 2, size / 2);
	let mut dir = 0; // 0 = right, 1 = down, 2 = left, 3 = up
	let mut seg_len = 2;
	let mut seg_rem = 1;
	for i in 1..(size * size + 1) {
		spiral[pos.0][pos.1] = i as u64;

		seg_rem -= 1;
		pos = match dir {
			0 => (pos.0, pos.1 + 1),
			1 => (pos.0 + 1, pos.1),
			2 => (pos.0, pos.1 - 1),
			3 => (pos.0 - 1, pos.1),
			_ => pos
		};

		if seg_rem == 0 {
			seg_len += 1;
			seg_rem = seg_len / 2;
			dir = (dir + 1) % 4; 
		}
	}

	spiral
}

#[allow(dead_code)]
pub fn get_answer() -> u64 {
	let size = 1001;
	let spiral = make_spiral(size);
	let down = get_diagonal_sum(&spiral, (0, 0), (1, 1), size);
	let up = get_diagonal_sum(&spiral, (size - 1, 0), (-1, 1), size);
	let sum = up + down - 1;
	sum
}
