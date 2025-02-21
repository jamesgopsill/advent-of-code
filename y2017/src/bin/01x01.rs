use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2017/01.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut values: Vec<u32> = vec![];
	for c in input.trim().chars() {
		let v = c.to_digit(10).unwrap();
		values.push(v);
	}
	values.push(values[0]);
	let mut sum: u32 = 0;
	for w in values.windows(2) {
		if w[0] == w[1] {
			sum += w[0];
		}
	}
	sum.to_string()
}

#[cfg(test)]
mod tests_0101 {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "1122";
		let result = invoke(input);
		assert_eq!(result, "3");
	}

	#[test]
	fn test_b() {
		let input = "1111";
		let result = invoke(input);
		assert_eq!(result, "4");
	}

	#[test]
	fn test_c() {
		let input = "1234";
		let result = invoke(input);
		assert_eq!(result, "0");
	}

	#[test]
	fn test_d() {
		let input = "91212129";
		let result = invoke(input);
		assert_eq!(result, "9");
	}
}
