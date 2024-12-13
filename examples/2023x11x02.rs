use advent::{bench::bench_val, y2023::t11x02::invoke};
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2023/11.txt").unwrap();
	let out = invoke(&input, 1_000_000);
	println!("{}", out);
	bench_val(invoke, &input, 1_000_000);
}
