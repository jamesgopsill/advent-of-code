use advent::y2024::t22x01::invoke;
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/22.txt").unwrap();
	let out = invoke(&input, 2_000);
	println!("{}", out);
	// bench(invoke, &input);
}
