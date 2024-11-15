use std::collections::HashSet;

pub fn invoke(input: String) -> usize {
	let mut locations_visited: HashSet<[i32; 2]> = HashSet::new();
	let mut location: [i32; 2] = [0, 0];
	locations_visited.insert(location.clone());

	for c in input.chars() {
		match c {
			'<' => location[0] -= 1,
			'>' => location[0] += 1,
			'^' => location[1] -= 1,
			'v' => location[1] += 1,
			_ => {}
		}
		locations_visited.insert(location.clone());
	}

	return locations_visited.len();
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(">".to_string());
		assert_eq!(result, 2);
	}

	#[test]
	fn test_b() {
		let result = invoke("^>v<".to_string());
		assert_eq!(result, 4);
	}

	#[test]
	fn test_c() {
		let result = invoke("^v^v^v^v^v".to_string());
		assert_eq!(result, 2);
	}
}
