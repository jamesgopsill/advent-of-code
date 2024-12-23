use advent::{bench::bench, y2017::t02x02::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2017/02.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}
