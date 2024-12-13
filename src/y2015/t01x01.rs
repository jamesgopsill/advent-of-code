pub fn invoke(input: &String) -> String {
	let mut floor: i32 = 0;
	for c in input.chars() {
		match c {
			'(' => floor += 1,
			')' => floor -= 1,
			_ => {}
		}
	}
	floor.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(&"(())".to_string());
		assert_eq!(result, "0");
	}

	#[test]
	fn test_b() {
		let result = invoke(&"()()".to_string());
		assert_eq!(result, "0");
	}

	#[test]
	fn test_c() {
		let result = invoke(&"(((".to_string());
		assert_eq!(result, "3");
	}
}
