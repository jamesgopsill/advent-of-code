pub fn invoke(input: &String) -> String {
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
	sum.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test() {
		let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
			.to_string();
		let result = invoke(&input);
		assert_eq!(result, "142");
	}
}
