use std::collections::{HashMap, HashSet};

use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/08.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut char_map: HashMap<char, Vec<[i32; 2]>> = HashMap::new();
	let mut row_max: i32 = 0;
	let mut col_max: i32 = 0;

	// Identify all character locations
	for (i, line) in input.lines().enumerate() {
		let i = i as i32;
		row_max = i;
		for (j, c) in line.chars().enumerate() {
			let j = j as i32;
			col_max = j;
			match c {
				'.' => {}
				_ => {
					let list = char_map.get_mut(&c);
					match list {
						Some(list) => list.push([i, j]),
						None => {
							let list = vec![[i, j]];
							char_map.insert(c, list);
						}
					}
				}
			}
		}
	}

	// Now iterate and identify locations on the map.
	let mut antinodes: HashSet<[i32; 2]> = HashSet::new();
	for (_, list) in char_map.iter() {
		for i in 0..list.len() {
			for j in i + 1..list.len() {
				let loc_a = &list[i];
				let loc_b = &list[j];
				// diff about a
				let antinode_x = loc_a[0] - (loc_b[0] - loc_a[0]);
				let antinode_y = loc_a[1] - (loc_b[1] - loc_a[1]);
				antinodes.insert([antinode_x, antinode_y]);
				// diff about b
				let antinode_x = loc_b[0] - (loc_a[0] - loc_b[0]);
				let antinode_y = loc_b[1] - (loc_a[1] - loc_b[1]);
				antinodes.insert([antinode_x, antinode_y]);
			}
		}
	}

	let mut on_map: u32 = 0;
	for antinode in antinodes {
		if antinode[0] >= 0 && antinode[0] <= row_max && antinode[1] >= 0 && antinode[1] <= col_max
		{
			on_map += 1;
		}
	}

	on_map.to_string()
}

#[cfg(test)]
mod tests_08x01 {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
";
		let result = invoke(input);
		assert_eq!(result, "2");
	}

	#[test]
	fn test_b() {
		let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
		let result = invoke(input);
		assert_eq!(result, "14");
	}
}
