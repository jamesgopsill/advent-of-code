use std::collections::HashSet;

pub fn invoke(
	input: String,
	_debug: bool,
) -> usize {
	let mut visited: HashSet<[i32; 2]> = HashSet::new();
	let mut santa: [i32; 2] = [0, 0];
	let mut robo_santa: [i32; 2] = [0, 0];
	visited.insert(santa.clone());
	visited.insert(robo_santa.clone());

	for (i, c) in input.chars().enumerate() {
		let remainder = i % 2;
		match remainder {
			0 => {
				match c {
					'<' => santa[0] -= 1,
					'>' => santa[0] += 1,
					'^' => santa[1] -= 1,
					'v' => santa[1] += 1,
					_ => {}
				}
				visited.insert(santa.clone());
			}
			_ => {
				match c {
					'<' => robo_santa[0] -= 1,
					'>' => robo_santa[0] += 1,
					'^' => robo_santa[1] -= 1,
					'v' => robo_santa[1] += 1,
					_ => {}
				};
				visited.insert(robo_santa.clone());
			}
		}
	}

	return visited.len();
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("^v".to_string(), true);
		assert_eq!(result, 3);
	}

	#[test]
	fn test_b() {
		let result = invoke("^>v<".to_string(), true);
		assert_eq!(result, 3);
	}

	#[test]
	fn test_c() {
		let result = invoke("^v^v^v^v^v".to_string(), true);
		assert_eq!(result, 11);
	}
}
