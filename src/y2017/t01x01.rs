pub fn invoke(input: &String) -> String {
	let mut values: Vec<u32> = vec![];
	for c in input.trim().chars() {
		let v = c.to_digit(10).unwrap();
		values.push(v);
	}
	values.push(values[0]);
	let mut sum: u32 = 0;
	for w in values.windows(2) {
		if w[0] == w[1] {
			sum += w[0];
		}
	}
	sum.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "1122".to_string();
		let result = invoke(&input);
		assert_eq!(result, "3");
	}

	#[test]
	fn test_b() {
		let input = "1111".to_string();
		let result = invoke(&input);
		assert_eq!(result, "4");
	}

	#[test]
	fn test_c() {
		let input = "1234".to_string();
		let result = invoke(&input);
		assert_eq!(result, "0");
	}

	#[test]
	fn test_d() {
		let input = "91212129".to_string();
		let result = invoke(&input);
		assert_eq!(result, "9");
	}
}
