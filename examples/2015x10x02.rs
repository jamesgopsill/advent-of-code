use advent::{bench::bench_val, y2015::t10x01::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/10.txt").unwrap();
	let out = invoke(&input, 50);
	println!("{}", out);
	bench_val(invoke, &input, 50);
}
