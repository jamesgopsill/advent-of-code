#![allow(dead_code)]
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/20.txt").unwrap();
	let out = invoke(&input, 0);
	println!("{}", out);
}

fn invoke(
	_input: &str,
	_saving: i32,
) -> String {
	todo!()
}

fn race(_input: &str) -> u32 {
	todo!()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_a() {
		let input = "###############
#...#...12....#
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
###############";
		let result = race(input);
		assert_eq!(result, 72);
	}
}
