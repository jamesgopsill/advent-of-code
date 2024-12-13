pub fn invoke(input: &String) -> String {
	let mut map: Vec<Vec<char>> = vec![];
	for line in input.lines() {
		let mut row: Vec<char> = vec![];
		for c in line.chars() {
			row.push(c);
		}
		map.push(row);
	}
	let mut count: u32 = 0;
	let row_count = map.len();
	let col_count = map[0].len();
	for i in 0..row_count {
		for j in 0..col_count {
			match map[i][j] {
				'X' => {
					//println!("X Found: {} {}", i, j);
					// Can we check UP
					if i >= 3 {
						if map[i - 1][j] == 'M' && map[i - 2][j] == 'A' && map[i - 3][j] == 'S' {
							//println!("XMAS UP");
							count += 1;
						}
						// Check UP LEFT
						if j >= 3 {
							if map[i - 1][j - 1] == 'M'
								&& map[i - 2][j - 2] == 'A'
								&& map[i - 3][j - 3] == 'S'
							{
								//println!("XMAS UP LEFT");
								count += 1;
							}
						}
						// Check UP RIGHT
						if j <= col_count - 4 {
							if map[i - 1][j + 1] == 'M'
								&& map[i - 2][j + 2] == 'A'
								&& map[i - 3][j + 3] == 'S'
							{
								//println!("XMAS UP RIGHT");
								count += 1;
							}
						}
					}
					// Can we check DOWN
					if i <= row_count - 4 {
						if map[i + 1][j] == 'M' && map[i + 2][j] == 'A' && map[i + 3][j] == 'S' {
							//println!("XMAS UP DOWN");
							count += 1;
						}
						// check DOWN LEFT
						if j >= 3 {
							if map[i + 1][j - 1] == 'M'
								&& map[i + 2][j - 2] == 'A'
								&& map[i + 3][j - 3] == 'S'
							{
								//println!("XMAS UP DOWN LEFT");
								count += 1;
							}
						}
						// check DOWN RIGHT
						if j <= col_count - 4 {
							if map[i + 1][j + 1] == 'M'
								&& map[i + 2][j + 2] == 'A'
								&& map[i + 3][j + 3] == 'S'
							{
								//println!("XMAS UP DOWN RIGHT");
								count += 1;
							}
						}
					}
					// Can we check left
					if j >= 3 {
						if map[i][j - 1] == 'M' && map[i][j - 2] == 'A' && map[i][j - 3] == 'S' {
							//println!("XMAS LEFT");
							count += 1;
						}
					}
					// Can we check right
					if j <= col_count - 4 {
						if map[i][j + 1] == 'M' && map[i][j + 2] == 'A' && map[i][j + 3] == 'S' {
							//println!("XMAS RIGHT");
							count += 1;
						}
					}
				}
				_ => {}
			}
		}
	}
	count.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			&"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"
			.to_string(),
		);
		assert_eq!(result, "18");
	}
}
