use advent::{bench::bench_val, y2015::t11x01::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/11.txt").unwrap();
	let out = invoke(&input, 75);
	println!("{}", out);
	bench_val(invoke, &input, 75);
}
