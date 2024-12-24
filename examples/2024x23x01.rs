use advent::y2024::t23x01::invoke;
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/23.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	// bench(invoke, &input);
}
