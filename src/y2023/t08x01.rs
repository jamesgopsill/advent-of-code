use std::collections::HashMap;

use regex::Regex;

pub fn invoke(
	input: String,
	debug: bool,
) -> u32 {
	let mut lines = input.lines();
	let instructions = lines.next().unwrap();
	if debug {
		dbg!(instructions);
	}
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

	steps
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn test_a() {
		let input = fs::read_to_string("test_data/2023/08x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input, true);
		assert_eq!(result, 2);
	}

	#[test]
	fn test_b() {
		let input = fs::read_to_string("test_data/2023/08x02.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input, true);
		assert_eq!(result, 6);
	}
}
