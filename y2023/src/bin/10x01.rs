use core::panic;
use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2023/10.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let lines: Vec<&str> = input.lines().collect();
	let mut map: Vec<Vec<char>> = vec![];

	// Step 1. Create the map
	for line in lines.iter() {
		let mut r = vec![];
		for char in line.chars() {
			r.push(char);
		}
		map.push(r);
	}

	let mut traveller = Traveller::new(map);
	traveller.find_start();

	println!("{:?}", traveller.current_tile);

	while traveller.step() {
		println!("{:?}", traveller.current_tile);
	}

	let distance: u32 = traveller.past_tiles.len() as u32 + 1;
	(distance / 2).to_string()
}

#[derive(Debug, PartialEq, Clone)]
struct Tile {
	row: usize,
	col: usize,
	char: char,
}

impl Tile {
	pub fn new(
		row: usize,
		col: usize,
		char: char,
	) -> Self {
		Self { row, col, char }
	}
}

struct Traveller {
	current_tile: Tile,
	past_tiles: Vec<Tile>,
	map: Vec<Vec<char>>,
}

impl Traveller {
	pub fn new(map: Vec<Vec<char>>) -> Self {
		Self {
			current_tile: Tile::new(0, 0, '*'),
			past_tiles: vec![],
			map,
		}
	}

	pub fn find_start(&mut self) {
		for (row, chars) in self.map.iter().enumerate() {
			for (col, char) in chars.iter().enumerate() {
				if *char == 'S' {
					self.current_tile = Tile::new(row, col, *char);
				}
			}
		}
	}

	pub fn step(&mut self) -> bool {
		match self.current_tile.char {
			'S' => {
				// Check all the possible valid positions
				if let Some(t) = self.tile_above() {
					let chars = ['7', '|', 'F'];
					if chars.contains(&t.char) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				if let Some(t) = self.tile_left() {
					let chars = ['L', '-', 'F'];
					if chars.contains(&t.char) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				if let Some(t) = self.tile_right() {
					let chars = ['7', '-', 'J'];
					if chars.contains(&t.char) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				if let Some(t) = self.tile_below() {
					let chars = ['J', '|', 'L'];
					if chars.contains(&t.char) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				panic!("Do not find a move from start");
			}
			'|' => {
				if let Some(t) = self.tile_above() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				if let Some(t) = self.tile_below() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				false
			}
			'-' => {
				if let Some(t) = self.tile_left() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				if let Some(t) = self.tile_right() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				false
			}
			'L' => {
				if let Some(t) = self.tile_right() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				if let Some(t) = self.tile_above() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				false
			}
			'J' => {
				if let Some(t) = self.tile_above() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				if let Some(t) = self.tile_left() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				false
			}
			'7' => {
				if let Some(t) = self.tile_left() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				if let Some(t) = self.tile_below() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				false
			}
			'F' => {
				if let Some(t) = self.tile_right() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				if let Some(t) = self.tile_below() {
					if !self.past_tiles.contains(&t) {
						self.past_tiles.push(self.current_tile.clone());
						self.current_tile = t;
						return true;
					}
				}
				false
			}
			'.' => {
				panic!("No pipe here");
			}
			_ => {
				panic!("How did I get here!?");
			}
		}
	}

	pub fn tile_above(&self) -> Option<Tile> {
		if self.current_tile.row == 0 {
			return None;
		}
		let row = self.current_tile.row - 1;
		let col = self.current_tile.col;
		let char = self.map[row][col];
		Some(Tile::new(row, col, char))
	}

	pub fn tile_left(&self) -> Option<Tile> {
		if self.current_tile.col == 0 {
			return None;
		}
		let row = self.current_tile.row;
		let col = self.current_tile.col - 1;
		let char = self.map[row][col];
		Some(Tile::new(row, col, char))
	}

	pub fn tile_right(&self) -> Option<Tile> {
		if self.current_tile.col == self.map[0].len() - 1 {
			return None;
		}
		let row = self.current_tile.row;
		let col = self.current_tile.col + 1;
		let char = self.map[row][col];
		Some(Tile::new(row, col, char))
	}

	pub fn tile_below(&self) -> Option<Tile> {
		if self.current_tile.row == self.map.len() - 1 {
			return None;
		}
		let row = self.current_tile.row + 1;
		let col = self.current_tile.col;
		let char = self.map[row][col];
		Some(Tile::new(row, col, char))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_a() {
		let input = ".....
.S-7.
.|.|.
.L-J.
.....
";
		let result = invoke(input);
		assert_eq!(result, "4");
	}

	#[test]
	fn test_b() {
		let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
		let result = invoke(input);
		assert_eq!(result, "8");
	}
}
