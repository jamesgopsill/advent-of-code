use std::mem::swap;

pub fn invoke(
	input: &String,
	iterations: usize,
) -> String {
	let lines: Vec<&str> = input.lines().collect();
	let n = lines.len();
	// Create a larger map with the outer values
	// as fixed boundary conditions to not have to
	// worry about the edge cases.
	let mut current_map: Vec<Vec<u32>> = vec![vec![0; n + 2]; n + 2];
	let mut next_map: Vec<Vec<u32>> = vec![vec![0; n + 2]; n + 2];
	// Initialise the map
	for (i, line) in lines.iter().enumerate() {
		let i = i + 1;
		for (j, char) in line.chars().enumerate() {
			let j = j + 1;
			match char {
				'.' => current_map[i][j] = 0,
				'#' => current_map[i][j] = 1,
				_ => {}
			}
		}
	}
	// Now to animate
	for _ in 0..iterations {
		for i in 1..n + 1 {
			for j in 1..n + 1 {
				// above
				let mut count = current_map[i - 1][j - 1];
				count += current_map[i - 1][j];
				count += current_map[i - 1][j + 1];
				// middle
				count += current_map[i][j - 1];
				count += current_map[i][j + 1];
				// below
				count += current_map[i + 1][j - 1];
				count += current_map[i + 1][j];
				count += current_map[i + 1][j + 1];

				match current_map[i][j] {
					1 => match count {
						2 | 3 => next_map[i][j] = 1,
						_ => next_map[i][j] = 0,
					},
					0 => match count {
						3 => next_map[i][j] = 1,
						_ => next_map[i][j] = 0,
					},
					_ => {
						panic!("Should not get here")
					}
				}
			}
		}
		swap(&mut current_map, &mut next_map);
		for row in next_map.iter_mut() {
			row.fill(0);
		}
	}
	// Now what the answer it is
	let mut sum = 0;
	for row in current_map {
		for val in row {
			sum += val;
		}
	}
	sum.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			&".#.#.#
...##.
#....#
..#...
#.#..#
####.."
				.to_string(),
			4,
		);
		assert_eq!(result, "4");
	}
}
