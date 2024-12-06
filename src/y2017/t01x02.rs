pub fn invoke(input: &String) -> u32 {
	let mut values: Vec<u32> = vec![];
	for c in input.trim().chars() {
		let v = c.to_digit(10).unwrap();
		values.push(v);
	}
	let count = values.len();
	let middle = count / 2;
	values.extend_from_within(0..);
	let mut sum: u32 = 0;
	for i in 0..count {
		if values[i] == values[i + middle] {
			sum += values[i]
		}
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "1212".to_string();
		let result = invoke(&input);
		assert_eq!(result, 6);
	}

	#[test]
	fn test_b() {
		let input = "1221".to_string();
		let result = invoke(&input);
		assert_eq!(result, 0);
	}

	#[test]
	fn test_c() {
		let input = "123425".to_string();
		let result = invoke(&input);
		assert_eq!(result, 4);
	}

	#[test]
	fn test_d() {
		let input = "123123".to_string();
		let result = invoke(&input);
		assert_eq!(result, 12);
	}

	#[test]
	fn test_e() {
		let input = "12131415".to_string();
		let result = invoke(&input);
		assert_eq!(result, 4);
	}
}
