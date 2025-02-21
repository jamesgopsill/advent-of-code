use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/01.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut floor: i32 = 0;
	for c in input.chars() {
		match c {
			'(' => floor += 1,
			')' => floor -= 1,
			_ => {}
		}
	}
	floor.to_string()
}

#[cfg(test)]
mod tests_0101 {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("(())");
		assert_eq!(result, "0");
	}

	#[test]
	fn test_b() {
		let result = invoke("()()");
		assert_eq!(result, "0");
	}

	#[test]
	fn test_c() {
		let result = invoke("(((");
		assert_eq!(result, "3");
	}
}
