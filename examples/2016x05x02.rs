use advent::y2016::t05x02::invoke;
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2016/05.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
}
