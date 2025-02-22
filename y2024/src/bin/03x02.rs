use regex::Regex;

use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/03.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let re = Regex::new(r"do\(\)|mul\(([\d]{1,3}),([\d]{1,3})\)|don\'t\(\)").unwrap();
	let instructions = re.captures_iter(input);
	let mut sum: u32 = 0;
	let mut enabled: bool = true;
	for instruction in instructions {
		let i = instruction.get(0).unwrap().as_str();
		if i.contains("don't") {
			enabled = false
		} else if i.contains("do") {
			enabled = true
		} else if enabled {
			let a = instruction.get(1).unwrap().as_str().parse::<u32>().unwrap();
			let b = instruction.get(2).unwrap().as_str().parse::<u32>().unwrap();
			// println!("{} * {}", a, b);
			sum += a * b;
		}
	}
	sum.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result =
			invoke("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
		assert_eq!(result, "48");
	}
}
