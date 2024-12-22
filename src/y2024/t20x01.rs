pub fn invoke(
	input: &String,
	saving: i32,
) -> String {
	let mut map: Vec<Vec<char>> = vec![];
	let mut start_row = 0;
	let mut start_col = 0;
	for (i, line) in input.lines().enumerate() {
		let mut row: Vec<char> = vec![];
		for (j, c) in line.chars().enumerate() {
			row.push(c);
			match c {
				'S' => {
					start_row = i;
					start_col = j;
				}
				_ => {}
			}
		}
		map.push(row)
	}

	let mut no_cheat = Historian::new(start_row, start_col);
	while no_cheat.finished == false {
		no_cheat = no_cheat.step(&map)[0].clone();
	}
	println!("No Cheat Time: {}", no_cheat.time);

	/*
	let mut cheat_map = map.clone();
	match cheat_map[0][0] {
		'E' => {}
		_ => cheat_map[0][0] = '.',
	}
	match cheat_map[0][0] {
		'E' => {}
		_ => cheat_map[0][0] = '.',
	}

	let h = Historian::new(start_row, start_col);
	let mut historians: Vec<Historian> = vec![h];
	while historians.len() > 0 {
		let mut new_hist: Vec<Historian> = vec![];
		for h in historians.iter() {
			let next_hist = h.step(&map);
			new_hist.extend(next_hist);
		}
		let mut flag = false;
		for nh in new_hist.iter() {
			if nh.finished {
				println!("Finished");
				println!("{:?}", nh);
				flag = true;
				//fastest_historians.push(nh.clone())
			}
		}
		if flag {
			break;
		}
		historians.clear();
		historians.extend(new_hist);
	}
	*/

	let mut fastest_historians: Vec<Historian> = vec![];
	for row in 0..map.len() - 1 {
		for col in 0..map[0].len() - 1 {
			let mut cheat_map = map.clone();
			match cheat_map[row][col] {
				'E' => {}
				_ => cheat_map[row][col] = '.',
			}
			match cheat_map[row + 1][col] {
				'E' => {}
				_ => cheat_map[row + 1][0] = '.',
			}

			//
			let h = Historian::new(start_row, start_col);
			let mut historians: Vec<Historian> = vec![h];
			while historians.len() > 0 {
				let mut new_hist: Vec<Historian> = vec![];
				for h in historians.iter() {
					let next_hist = h.step(&cheat_map);
					new_hist.extend(next_hist);
				}
				let mut flag = false;
				for nh in new_hist.iter() {
					if nh.finished {
						flag = true;
						fastest_historians.push(nh.clone())
					}
				}
				if flag {
					break;
				}
				historians.clear();
				historians.extend(new_hist);
			}

			let mut cheat_map = map.clone();
			match cheat_map[row][col] {
				'E' => {}
				_ => cheat_map[row][col] = '.',
			}
			match cheat_map[row][col + 1] {
				'E' => {}
				_ => cheat_map[row][col + 1] = '.',
			}

			//
			let h = Historian::new(start_row, start_col);
			let mut historians: Vec<Historian> = vec![h];
			while historians.len() > 0 {
				let mut new_hist: Vec<Historian> = vec![];
				for h in historians.iter() {
					let next_hist = h.step(&cheat_map);
					new_hist.extend(next_hist);
				}
				let mut flag = false;
				for nh in new_hist.iter() {
					if nh.finished {
						flag = true;
						fastest_historians.push(nh.clone())
					}
				}
				if flag {
					break;
				}
				historians.clear();
				historians.extend(new_hist);
			}
		}
	}

	let mut cheat_count: u32 = 0;
	for fh in fastest_historians.iter() {
		let saved = no_cheat.time - fh.time;
		println!("{} {} {}", no_cheat.time, fh.time, saved);
		if saved == saving {
			cheat_count += 1;
		}
	}

	cheat_count.to_string()
}

#[derive(Clone, Debug)]
struct Historian {
	row: usize,
	col: usize,
	time: i32,
	finished: bool,
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
			time: 0,
			finished: false,
			visited: vec![(row, col)],
		}
	}

	fn check_new_location(
		&self,
		row: usize,
		col: usize,
		map: &Vec<Vec<char>>,
	) -> Option<Historian> {
		let map_row = map.get(row);
		if map_row.is_none() {
			return None;
		}
		let map_row = map_row.unwrap();
		let val = map_row.get(col);
		if val.is_none() {
			return None;
		}
		let val = val.unwrap();

		if self.visited.contains(&(row, col)) {
			return None;
		}

		let time = self.time + 1;

		let mut h = self.clone();
		h.row = row;
		h.col = col;
		h.time = time;
		h.visited.push((h.row, h.col));

		match val {
			'.' => return Some(h),
			'E' => {
				h.finished = true;
				return Some(h);
			}
			_ => return None,
		}
	}

	fn step(
		&self,
		map: &Vec<Vec<char>>,
	) -> Vec<Historian> {
		let mut historians: Vec<Historian> = vec![];

		// top
		if self.row != 0 {
			if let Some(h) = self.check_new_location(self.row - 1, self.col, map) {
				historians.push(h)
			}
		}

		// right
		if let Some(h) = self.check_new_location(self.row, self.col + 1, map) {
			historians.push(h)
		}

		// bottom
		if let Some(h) = self.check_new_location(self.row + 1, self.col, map) {
			historians.push(h)
		}

		// left
		if self.col != 0 {
			if let Some(h) = self.check_new_location(self.row, self.col - 1, map) {
				historians.push(h)
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
		let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############"
			.to_string();
		let result = invoke(&input, 2);
		assert_eq!(result, "14");
	}
}
