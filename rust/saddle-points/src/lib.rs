/// Copyright © Geoff Maggi 2018
/// HW 2 - CS510 Rust Spring 2018
///   Specs: http://exercism.io/ - Saddle Points

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
	let mut saddle_points = Vec::new();
	for (i, row) in input.iter().enumerate() {
		let row_max = row.iter().max().unwrap_or(&0);
		for (j, &curr) in row.iter().enumerate() {
			//println!("{:?}, {:?}", i, j);
			if curr >= *row_max {
				let col_min = (input.iter().map(|row| row[j])).min().unwrap_or(0);
				if curr <= col_min {
					saddle_points.push((i,j));
				}
			}
		}
	}
	saddle_points
}
