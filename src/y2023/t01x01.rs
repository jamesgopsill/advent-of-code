pub fn invoke(
	input: String,
	debug: bool,
) -> u32 {
	let mut sum: u32 = 0;
	let lines = input.lines();
	for line in lines {
		let digits: Vec<&str> = line.matches(char::is_numeric).collect();
		let first = digits.first().unwrap();
		let last = digits.last().unwrap();
		let number = format!("{}{}", first, last).parse::<u32>().unwrap();
		if debug {
			dbg!(line, number);
		}
		sum += number;
	}
	sum
}

#[cfg(test)]
mod tests {
	use std::{env::current_dir, fs};

	use super::invoke;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/2023/01x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input, true);
		assert_eq!(result, 142);
	}
}
