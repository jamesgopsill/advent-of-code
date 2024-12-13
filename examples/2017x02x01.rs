use advent::{bench::bench, y2017::t02x01::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2017/01.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}
