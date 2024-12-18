use std::{u64, vec};

// Not working on main puzzle.
pub fn invoke(input: &String) -> String {
	let mut m: Vec<Vec<(char, u64)>> = vec![];
	for row in input.lines() {
		let mut r: Vec<(char, u64)> = vec![];
		for c in row.chars() {
			r.push((c, u64::MAX))
		}
		m.push(r);
	}
	let mut map = Map::new(m);
	// map.print();

	let row_max = map.row_len();
	let col_max = map.col_len();
	let mut deer: Vec<Reindeer> = vec![];
	for i in 0..row_max {
		for j in 0..col_max {
			let (obs, _score) = map.get(i, j);
			match obs {
				'S' => {
					let r = Reindeer::new(i, j, Facing::East, 0);
					deer.push(r);
					map.0[i][j].1 = 0;
				}
				_ => {}
			}
		}
	}

	while deer.len() > 0 {
		let mut next_deer: Vec<Reindeer> = vec![];
		while let Some(d) = deer.pop() {
			let nd = d.next(&mut map);
			next_deer.extend(nd)
		}
		// println!("Next Deer: {}", next_deer.len());
		deer.extend(next_deer);
	}

	map.print_scores();

	// Best path
	//

	let mut deer: Vec<Reindeer> = vec![];
	for i in 0..row_max {
		for j in 0..col_max {
			let (obs, score) = map.get(i, j);
			match obs {
				'E' => {
					// set the deers off to avoid the issue in test_b
					if map.0[i - 1][j].1 == score - 1 {
						let r = Reindeer::new(i - 1, j, Facing::North, score - 1);
						map.set_char(i - 1, j, 'O');
						deer.push(r);
					}
					if map.0[i + 1][j].1 == score - 1 {
						let r = Reindeer::new(i + 1, j, Facing::North, score - 1);
						map.set_char(i + 1, j, 'O');
						deer.push(r);
					}
					if map.0[i][j + 1].1 == score - 1 {
						let r = Reindeer::new(i, j + 1, Facing::North, score - 1);
						map.set_char(i, j + 1, 'O');
						deer.push(r);
					}
					if map.0[i][j - 1].1 == score - 1 {
						let r = Reindeer::new(i, j - 1, Facing::North, score - 1);
						map.set_char(i, j - 1, 'O');
						deer.push(r);
					}
				}
				_ => {}
			}
		}
	}

	while deer.len() > 0 {
		let mut next_deer: Vec<Reindeer> = vec![];
		while let Some(d) = deer.pop() {
			let nd = d.prev(&mut map);
			next_deer.extend(nd)
		}
		// println!("Next Deer: {}", next_deer.len());
		deer.extend(next_deer);
	}

	map.print();

	let mut ans: u32 = 2; // S & E
	for i in 0..row_max {
		for j in 0..col_max {
			let (obs, _score) = map.get(i, j);
			match obs {
				'O' => ans += 1,
				_ => {}
			}
		}
	}

	ans.to_string()
}

struct Map(Vec<Vec<(char, u64)>>);

impl Map {
	fn new(map: Vec<Vec<(char, u64)>>) -> Self {
		Self(map)
	}

	fn row_len(&self) -> usize {
		self.0.len()
	}

	fn col_len(&self) -> usize {
		self.0[0].len()
	}

	fn get(
		&self,
		row: usize,
		col: usize,
	) -> (char, u64) {
		self.0[row][col]
	}

	fn set_score(
		&mut self,
		row: usize,
		col: usize,
		score: u64,
	) {
		self.0[row][col].1 = score
	}

	fn set_char(
		&mut self,
		row: usize,
		col: usize,
		c: char,
	) {
		self.0[row][col].0 = c
	}

	fn print(&self) {
		for row in self.0.iter() {
			for c in row {
				print!("{}", c.0);
			}
			println!("");
		}
	}

	fn print_scores(&self) {
		for row in self.0.iter() {
			for c in row {
				if c.1 == u64::MAX {
					print!("(####)");
				} else {
					print!("({})", c.1);
				}
			}
			println!("");
		}
	}
}

#[derive(Clone, PartialEq, Debug)]
enum Facing {
	North,
	East,
	South,
	West,
}

#[derive(Clone)]
struct Reindeer {
	row: usize,
	col: usize,
	facing: Facing,
	score: u64,
}

impl Reindeer {
	fn new(
		row: usize,
		col: usize,
		facing: Facing,
		score: u64,
	) -> Self {
		Self {
			row,
			col,
			facing,
			score,
		}
	}

	fn next(
		&self,
		map: &mut Map,
	) -> Vec<Reindeer> {
		// row, col, facing, score
		let mut moves: Vec<(usize, usize, Facing, u64)> = vec![];
		match self.facing {
			Facing::North => {
				moves.push((self.row - 1, self.col, Facing::North, self.score + 1));
				moves.push((self.row, self.col + 1, Facing::East, self.score + 1001));
				moves.push((self.row + 1, self.col, Facing::South, self.score + 2001));
				moves.push((self.row, self.col - 1, Facing::West, self.score + 1001));
			}
			Facing::East => {
				moves.push((self.row - 1, self.col, Facing::North, self.score + 1001));
				moves.push((self.row, self.col + 1, Facing::East, self.score + 1));
				moves.push((self.row + 1, self.col, Facing::South, self.score + 1001));
				moves.push((self.row, self.col - 1, Facing::West, self.score + 2001));
			}
			Facing::South => {
				moves.push((self.row - 1, self.col, Facing::North, self.score + 2001));
				moves.push((self.row, self.col + 1, Facing::East, self.score + 1001));
				moves.push((self.row + 1, self.col, Facing::South, self.score + 1));
				moves.push((self.row, self.col - 1, Facing::West, self.score + 1001));
			}
			Facing::West => {
				moves.push((self.row - 1, self.col, Facing::North, self.score + 1001));
				moves.push((self.row, self.col + 1, Facing::East, self.score + 2001));
				moves.push((self.row + 1, self.col, Facing::South, self.score + 1001));
				moves.push((self.row, self.col - 1, Facing::West, self.score + 1));
			}
		}

		let mut deer: Vec<Self> = vec![];
		for (row, col, facing, score) in moves {
			let (c, s) = map.get(row, col);
			match c {
				'.' => {
					if score < s {
						map.set_score(row, col, score);
						let r = Reindeer::new(row, col, facing, score);
						deer.push(r);
					}
				}
				'E' => {
					if score < s {
						map.set_score(row, col, score);
					}
				}
				_ => {}
			}
		}
		deer
	}

	fn prev(
		&self,
		map: &mut Map,
	) -> Vec<Reindeer> {
		// row, col, facing, score
		let mut moves: Vec<(usize, usize, Facing, u64)> = vec![];
		if let Some(s) = self.score.checked_sub(1) {
			moves.push((self.row - 1, self.col, Facing::North, s));
			moves.push((self.row, self.col + 1, Facing::North, s));
			moves.push((self.row + 1, self.col, Facing::North, s));
			moves.push((self.row, self.col - 1, Facing::North, s));
		}
		if let Some(s) = self.score.checked_sub(1001) {
			moves.push((self.row - 1, self.col, Facing::North, s));
			moves.push((self.row, self.col + 1, Facing::North, s));
			moves.push((self.row + 1, self.col, Facing::North, s));
			moves.push((self.row, self.col - 1, Facing::North, s));
		}
		if let Some(s) = self.score.checked_sub(2001) {
			moves.push((self.row - 1, self.col, Facing::North, s));
			moves.push((self.row, self.col + 1, Facing::North, s));
			moves.push((self.row + 1, self.col, Facing::North, s));
			moves.push((self.row, self.col - 1, Facing::North, s));
		}
		if let Some(s) = self.score.checked_add(999) {
			moves.push((self.row - 1, self.col, Facing::North, s));
			moves.push((self.row, self.col + 1, Facing::North, s));
			moves.push((self.row + 1, self.col, Facing::North, s));
			moves.push((self.row, self.col - 1, Facing::North, s));
		}

		let mut deer: Vec<Self> = vec![];
		println!("----");
		for (row, col, facing, score) in moves {
			let (c, s) = map.get(row, col);
			println!("{} {} {:?}", score, s, score.checked_sub(s));
			match c {
				'.' => {
					// Then we're on a critical path.
					if score == s {
						map.set_char(row, col, 'O');
						let r = Reindeer::new(row, col, facing, score);
						deer.push(r);
					}
				}
				_ => {}
			}
		}
		deer
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"
		.to_string();
		let result = invoke(&input);
		assert_eq!(result, "45");
	}

	#[test]
	fn test_b() {
		let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
			.to_string();
		let result = invoke(&input);
		assert_eq!(result, "64");
	}
}
