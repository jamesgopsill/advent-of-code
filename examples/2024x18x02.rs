use advent::y2024::t18x02::invoke;
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/18.txt").unwrap();
	let out = invoke(&input, 70, 70);
	println!("{}", out);
	// bench(invoke, &input);
}
