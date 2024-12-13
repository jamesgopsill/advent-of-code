use advent::{bench::bench, y2015::t13x01::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/13.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}
