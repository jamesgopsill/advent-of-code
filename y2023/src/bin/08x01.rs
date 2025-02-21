use regex::Regex;
use std::collections::HashMap;
use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2023/08.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut lines = input.lines();
	let instructions = lines.next().unwrap();
	// dbg!(instructions);
	lines.next().unwrap();

	let re = Regex::new(r"\w+").unwrap();
	let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
	for line in lines {
		let mut finds = re.find_iter(line);
		let key = finds.next().unwrap().as_str();
		let left = finds.next().unwrap().as_str();
		let right = finds.next().unwrap().as_str();
		map.insert(key, (left, right));
	}

	let mut steps: u32 = 0;
	let mut i_idx: usize = 0;
	let i_len = instructions.len();
	let mut loc = "AAA";

	loop {
		steps += 1;
		let char = instructions.chars().nth(i_idx).unwrap();
		let (left, right) = map.get(loc).unwrap();
		match char {
			'L' => loc = left,
			'R' => loc = right,
			_ => {}
		}
		if loc == "ZZZ" {
			break;
		}
		i_idx += 1;
		if i_idx == i_len {
			i_idx = 0;
		}
	}

	steps.to_string()
}

#[cfg(test)]
mod tests_0801 {
	use super::*;

	#[test]
	fn test_a() {
		let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
		let result = invoke(input);
		assert_eq!(result, "2");
	}

	#[test]
	fn test_b() {
		let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
		let result = invoke(input);
		assert_eq!(result, "6");
	}
}
