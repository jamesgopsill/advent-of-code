use std::collections::HashMap;

use regex::Regex;

// TODO: too slow. Got to be efficient.
pub fn invoke(input: String) -> u32 {
	let mut lines = input.lines();
	let instructions = lines.next().unwrap();
	// dbg!(instructions);
	lines.next().unwrap();

	let re = Regex::new(r"\w+").unwrap();
	let mut map: HashMap<String, (String, String)> = HashMap::new();
	for line in lines {
		let mut finds = re.find_iter(line);
		let key = finds.next().unwrap().as_str().to_string();
		let left = finds.next().unwrap().as_str().to_string();
		let right = finds.next().unwrap().as_str().to_string();
		map.insert(key, (left, right));
	}

	// Find the start points
	let mut locs: Vec<&String> = vec![];
	for key in map.keys() {
		if key.ends_with("A") {
			locs.push(key)
		}
	}

	println!("{:?}", locs);

	let loc_count: usize = locs.len();
	let mut steps: u32 = 0;
	let mut i_idx: usize = 0;
	let i_len = instructions.len();

	loop {
		steps += 1;
		let char = instructions.chars().nth(i_idx).unwrap();
		let mut count: usize = 0;
		// Move around and check if they have
		// reached the end
		for loc in locs.iter_mut() {
			let (left, right) = map.get(*loc).unwrap();
			match char {
				'L' => {
					if left.ends_with("Z") {
						count += 1
					}
					*loc = left
				}
				'R' => {
					if right.ends_with("Z") {
						count += 1
					}
					*loc = right
				}
				_ => {}
			}
		}
		if count == loc_count {
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

	#[test]
	fn test() {
		let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
			.to_string();
		let result = invoke(input);
		assert_eq!(result, 6);
	}
}
