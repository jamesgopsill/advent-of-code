pub fn invoke(input: String) -> u32 {
	let mut valid: u32 = 0;
	for line in input.lines() {
		let mut items = line.split_whitespace();
		let a = items.next().unwrap().parse::<u32>().unwrap();
		let b = items.next().unwrap().parse::<u32>().unwrap();
		let c = items.next().unwrap().parse::<u32>().unwrap();
		if a + b > c && b + c > a && c + a > b {
			// println!("{}", line);
			valid += 1;
		}
	}
	valid
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("5 10 25".to_string());
		assert_eq!(result, 0);
	}
}
