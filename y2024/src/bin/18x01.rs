use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/18.txt").unwrap();
	let out = invoke(&input, 70, 70, 1024);
	println!("{}", out);
}

fn invoke(
	input: &str,
	row_max: usize,
	col_max: usize,
	time: usize,
) -> String {
	let mut map = Map::new(row_max, col_max);
	for (i, line) in input.lines().enumerate() {
		if time == i {
			break;
		}
		let (row, col) = line.split_once(",").unwrap();
		let x = row.parse::<usize>().unwrap();
		let y = col.parse::<usize>().unwrap();
		map.set(y, x, u32::MAX);
	}
	map.print();

	map.0[0][0] = 0;
	let h = Historian::new(0, 0);
	let mut historians: Vec<Historian> = vec![h];
	for _ in 0..1_000_000 {
		let mut next_historians: Vec<Historian> = vec![];
		for h in historians.iter() {
			next_historians.extend(h.walk(&mut map));
		}
		historians.clear();
		for h in next_historians.iter() {
			if !(h.row == map.row_len() - 1 && h.col == map.col_len() - 1) {
				historians.push(h.clone());
			}
		}
	}

	map.print_scores();

	map.0[row_max][col_max].to_string()
}

struct Map(Vec<Vec<u32>>);

impl Map {
	fn new(
		row_max: usize,
		col_max: usize,
	) -> Self {
		let mut map: Vec<Vec<u32>> = vec![];
		for _ in 0..=row_max {
			let mut row: Vec<u32> = vec![];
			for _ in 0..=col_max {
				row.push(u32::MAX - 1)
			}
			map.push(row);
		}
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
	) -> u32 {
		self.0[row][col]
	}

	fn set(
		&mut self,
		row: usize,
		col: usize,
		val: u32,
	) {
		self.0[row][col] = val
	}

	fn print(&self) {
		for row in self.0.iter() {
			for c in row {
				if *c == u32::MAX {
					print!("#")
				} else {
					print!(".")
				}
			}
			println!();
		}
	}

	fn print_scores(&self) {
		for row in self.0.iter() {
			for c in row {
				if *c == u32::MAX {
					print!("(##)");
				} else if *c == u32::MAX - 1 {
					print!("(!!)");
				} else {
					print!("({})", c);
				}
			}
			println!();
		}
	}
}

#[derive(Clone, Debug)]
struct Historian {
	row: usize,
	col: usize,
	score: u32,
	visited: Vec<(usize, usize)>,
}

impl Historian {
	fn new(
		row: usize,
		col: usize,
	) -> Self {
		Self {
			row,
			col,
			score: 0,
			visited: vec![(row, col)],
		}
	}

	fn walk(
		&self,
		map: &mut Map,
	) -> Vec<Historian> {
		let mut historians: Vec<Historian> = vec![];
		let new_score = self.score + 1;
		// top
		if self.row != 0 && !self.visited.contains(&(self.row - 1, self.col)) {
			let val = map.get(self.row - 1, self.col);
			if new_score < val && val != u32::MAX {
				let mut h = self.clone();
				h.row -= 1;
				h.score = new_score;
				h.visited.push((h.row, h.col));
				map.set(h.row, h.col, h.score);
				historians.push(h);
			}
		}
		// right
		if self.col != map.col_len() - 1 && !self.visited.contains(&(self.row, self.col + 1)) {
			let val = map.get(self.row, self.col + 1);
			if new_score < val && val != u32::MAX {
				let mut h = self.clone();
				h.col += 1;
				h.score = new_score;
				h.visited.push((h.row, h.col));
				map.set(h.row, h.col, h.score);
				historians.push(h);
			}
		}
		// bottom
		if self.row != map.row_len() - 1 && !self.visited.contains(&(self.row + 1, self.col)) {
			let val = map.get(self.row + 1, self.col);
			if new_score < val && val != u32::MAX {
				let mut h = self.clone();
				h.row += 1;
				h.score = new_score;
				h.visited.push((h.row, h.col));
				map.set(h.row, h.col, h.score);
				historians.push(h);
			}
		}
		// left
		if self.col != 0 && !self.visited.contains(&(self.row, self.col - 1)) {
			let val = map.get(self.row, self.col - 1);
			if new_score < val && val != u32::MAX {
				let mut h = self.clone();
				h.col -= 1;
				h.score = new_score;
				h.visited.push((h.row, h.col));
				map.set(h.row, h.col, h.score);
				historians.push(h);
			}
		}
		historians
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
		let result = invoke(input, 6, 6, 12);
		assert_eq!(result, "22");
	}
}
