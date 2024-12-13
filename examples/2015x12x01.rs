use advent::{bench::bench, y2015::t12x01::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/12.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}
