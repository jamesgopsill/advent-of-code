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
	for (i, c) in input.chars().enumerate() {
		match c {
			'(' => floor += 1,
			')' => floor -= 1,
			_ => {}
		}
		if floor == -1 {
			return (i + 1).to_string();
		}
	}
	0.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(")");
		assert_eq!(result, "1");
	}

	#[test]
	fn test_b() {
		let result = invoke("()())");
		assert_eq!(result, "5");
	}
}
