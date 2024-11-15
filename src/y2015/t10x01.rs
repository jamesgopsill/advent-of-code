pub fn invoke(
	input: String,
	times: u32,
) -> u32 {
	// Rust regex does not support backreferencing -> (\w)\1+
	// https://stackoverflow.com/questions/12258622/regular-expression-to-check-for-repeating-characters#12258829
	// Could use another package but I think we could group this ourselves.

	let mut sequence = input.clone().trim().to_string();
	println!("{}", sequence);
	for _ in 0..times {
		// Identify the groups
		let mut groups: Vec<(char, u32)> = vec![];
		let mut current_char: char = 'Z';
		let mut count: u32 = 0;
		for char in sequence.chars() {
			if current_char == char {
				count += 1;
			} else {
				if current_char != 'Z' {
					let group = (current_char.clone(), count.clone());
					groups.push(group);
				}
				current_char = char;
				count = 1;
			}
		}
		// append the last group
		let group = (current_char.clone(), count.clone());
		groups.push(group);

		// Create the new sequence
		let mut new_sequence = String::new();
		for (char, count) in groups {
			let count = format!("{}", count);
			new_sequence.push_str(count.as_str());
			new_sequence.push(char);
		}
		sequence = new_sequence
	}
	// println!("{}", sequence);
	sequence.len() as u32
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("1".to_string(), 1);
		assert_eq!(result, 2);
	}

	#[test]
	fn test_b() {
		let result = invoke("11".to_string(), 1);
		assert_eq!(result, 2);
	}

	#[test]
	fn test_c() {
		let result = invoke("21".to_string(), 1);
		assert_eq!(result, 4);
	}

	#[test]
	fn test_d() {
		let result = invoke("1211".to_string(), 1);
		assert_eq!(result, 6);
	}

	#[test]
	fn test_e() {
		let result = invoke("111221".to_string(), 1);
		assert_eq!(result, 6);
	}
}
