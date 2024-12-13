use advent::{bench::bench, y2023::t03x01::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2023/03.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}
