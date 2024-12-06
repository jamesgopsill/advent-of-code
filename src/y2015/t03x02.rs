use std::collections::HashSet;

pub fn invoke(input: String) -> usize {
	let mut santa = Santa::new();
	let mut robo_santa = Santa::new();
	for (i, c) in input.chars().enumerate() {
		if i % 2 == 0 {
			santa.step(c);
		} else {
			robo_santa.step(c);
		}
	}
	let houses: HashSet<_> = santa
		.unique_locations
		.union(&robo_santa.unique_locations)
		.collect();
	houses.len()
}

struct Santa {
	current_location: [i32; 2],
	unique_locations: HashSet<[i32; 2]>,
}

impl Santa {
	fn new() -> Self {
		let current_location = [0, 0];
		let mut unique_locations = HashSet::new();
		unique_locations.insert(current_location.clone());
		Self {
			current_location,
			unique_locations,
		}
	}

	fn step(
		&mut self,
		direction: char,
	) {
		match direction {
			'^' => self.current_location[1] += 1,
			'>' => self.current_location[0] += 1,
			'<' => self.current_location[0] -= 1,
			'v' => self.current_location[1] -= 1,
			_ => {}
		}
		self.unique_locations.insert(self.current_location.clone());
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("^v".to_string());
		assert_eq!(result, 3);
	}

	#[test]
	fn test_b() {
		let result = invoke("^>v<".to_string());
		assert_eq!(result, 3);
	}

	#[test]
	fn test_c() {
		let result = invoke("^v^v^v^v^v".to_string());
		assert_eq!(result, 11);
	}
}
