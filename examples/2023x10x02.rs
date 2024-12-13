use advent::{bench::bench, y2023::t10x02::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2023/10.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}
