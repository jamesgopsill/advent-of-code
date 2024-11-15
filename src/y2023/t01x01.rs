pub fn invoke(input: String) -> u32 {
	let mut sum: u32 = 0;
	let lines = input.lines();
	for line in lines {
		let digits: Vec<&str> = line.matches(char::is_numeric).collect();
		let first = digits.first().unwrap();
		let last = digits.last().unwrap();
		let number = format!("{}{}", first, last).parse::<u32>().unwrap();

		dbg!(line, number);

		sum += number;
	}
	sum
}

#[cfg(test)]
mod tests {
	use std::fs;

	use super::invoke;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/2023/01x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input);
		assert_eq!(result, 142);
	}
}
