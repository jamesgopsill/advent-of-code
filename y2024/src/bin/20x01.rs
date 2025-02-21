use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/20.txt").unwrap();
	let out = invoke(&input, 0);
}

fn invoke(
	input: &str,
	saving: i32,
) -> String {
	todo!()
}

fn race(input: &str) -> u32 {
	todo!()
}

#[cfg(test)]
mod tests_20x01 {
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
