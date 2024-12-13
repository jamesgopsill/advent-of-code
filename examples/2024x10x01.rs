use advent::{bench::bench, y2024::t10x01::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/10.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}
