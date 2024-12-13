use advent::{bench::bench_val, y2024::t11x02::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/11.txt").unwrap();
	let out = invoke(&input, 75);
	println!("{}", out);
	bench_val(invoke, &input, 75);
}
