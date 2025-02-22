use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2023/11.txt").unwrap();
	let out = invoke(&input, 1_000_000);
	println!("{}", out);
}

fn invoke(
	input: &str,
	expansion: u64,
) -> String {
	// Convert the image as a Vec<Vec<char>>
	let lines: Vec<&str> = input.lines().collect();
	let mut image: Vec<Vec<char>> = vec![];
	for line in lines.iter() {
		let mut row = vec![];
		for char in line.chars() {
			row.push(char);
		}
		image.push(row);
	}

	// Create the universe
	let mut universe = Universe::new(image);
	universe.identify_empty_rows();
	universe.identify_empty_cols();
	universe.identify_galaxies();

	let n_galaxies = universe.galaxies.len();

	let mut sum = 0;

	for i in 0..n_galaxies {
		for j in (i + 1)..n_galaxies {
			let g1 = universe.galaxies[i];
			let g2 = universe.galaxies[j];
			let mut row_sum = 0;
			if g2.0 > g1.0 {
				for idx in g1.0..g2.0 {
					if universe.empty_rows.contains(&idx) {
						row_sum += expansion;
					} else {
						row_sum += 1;
					}
				}
			} else {
				for idx in g2.0..g1.0 {
					if universe.empty_rows.contains(&idx) {
						row_sum += expansion;
					} else {
						row_sum += 1;
					}
				}
			}

			let mut col_sum = 0;
			if g2.1 > g1.1 {
				for idx in g1.1..g2.1 {
					if universe.empty_cols.contains(&idx) {
						col_sum += expansion;
					} else {
						col_sum += 1;
					}
				}
			} else {
				for idx in g2.1..g1.1 {
					if universe.empty_cols.contains(&idx) {
						col_sum += expansion;
					} else {
						col_sum += 1;
					}
				}
			}
			sum += row_sum;
			sum += col_sum;
		}
	}

	sum.to_string()
}

struct Universe {
	pub image: Vec<Vec<char>>,
	pub galaxies: Vec<(usize, usize)>,
	pub empty_rows: Vec<usize>,
	pub empty_cols: Vec<usize>,
}

impl Universe {
	pub fn new(image: Vec<Vec<char>>) -> Self {
		Self {
			image,
			galaxies: vec![],
			empty_rows: vec![],
			empty_cols: vec![],
		}
	}

	pub fn identify_empty_rows(&mut self) {
		for (i, row) in self.image.iter().enumerate() {
			if !row.contains(&'#') {
				self.empty_rows.push(i);
			}
		}
	}

	pub fn identify_empty_cols(&mut self) {
		let n_rows = self.image.len();
		let n_cols = self.image[0].len();

		for i in 0..n_cols {
			let mut empty_space = true;
			for j in 0..n_rows {
				if self.image[j][i] == '#' {
					empty_space = false;
					break;
				}
			}
			if empty_space {
				self.empty_cols.push(i);
			}
		}
	}

	pub fn identify_galaxies(&mut self) {
		for (row, chars) in self.image.iter().enumerate() {
			for (col, char) in chars.iter().enumerate() {
				if *char == '#' {
					self.galaxies.push((row, col));
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_a() {
		let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
		let result = invoke(input, 10);
		assert_eq!(result, "1030");
	}

	#[test]
	fn test_b() {
		let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
		let result = invoke(input, 100);
		assert_eq!(result, "8410");
	}
}
