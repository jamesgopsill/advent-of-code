use advent::{bench::bench, y2024::t01x02::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/01.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}
