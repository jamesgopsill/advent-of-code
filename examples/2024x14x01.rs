use advent::y2024::t14x01::invoke;
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/14.txt").unwrap();
	let out = invoke(&input, 101, 103);
	println!("{}", out);
	// bench(invoke, &input);
}
