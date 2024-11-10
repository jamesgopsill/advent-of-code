pub fn invoke(
	input: String,
	_debug: bool,
) -> usize {
	let mut floor: i32 = 0;
	for (i, c) in input.chars().enumerate() {
		match c {
			'(' => floor += 1,
			')' => floor -= 1,
			_ => {}
		}
		if floor == -1 {
			return i + 1;
		}
	}
	0
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(")".to_string(), true);
		assert_eq!(result, 1);
	}

	#[test]
	fn test_b() {
		let result = invoke("()())".to_string(), true);
		assert_eq!(result, 5);
	}
}
