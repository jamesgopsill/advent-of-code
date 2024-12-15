use advent::y2016::t08x01::invoke;
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2016/08.txt").unwrap();
	let out = invoke(&input, 6, 50);
	println!("{}", out);
	//bench(invoke, &input);
}
