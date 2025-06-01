use std::{collections::HashSet, fs};
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2018/01.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let changes: Vec<i32> = input
		.split_whitespace()
		.map(|v| v.trim().parse::<i32>().unwrap())
		.collect();

	let mut do_no_exit = true;
	let mut visited: HashSet<i32> = HashSet::new();
	let mut freq = 0;
	visited.insert(freq);
	while do_no_exit {
		for change in &changes {
			freq += change;
			if visited.contains(&freq) {
				do_no_exit = false;
				break;
			} else {
				visited.insert(freq);
			}
		}
	}

	freq.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "+1\n-1";
		let result = invoke(input);
		assert_eq!(result, "0");
	}

	#[test]
	fn test_b() {
		let input = "+3\n+3\n+4\n-2\n-4";
		let result = invoke(input);
		assert_eq!(result, "10");
	}

	#[test]
	fn test_c() {
		let input = "-6\n+3\n+8\n+5\n-6";
		let result = invoke(input);
		assert_eq!(result, "5");
	}

	#[test]
	fn test_d() {
		let input = "+7\n+7\n-2\n-7\n-4";
		let result = invoke(input);
		assert_eq!(result, "14");
	}
}
