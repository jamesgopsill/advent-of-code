pub fn invoke(input: &String) -> String {
	let mut sum: u32 = 0;
	for line in input.lines() {
		let items = line.split_whitespace().collect::<Vec<_>>();
		let values = items
			.iter()
			.map(|v| v.parse::<u32>().unwrap())
			.collect::<Vec<_>>();
		let min = values.iter().min().unwrap();
		let max = values.iter().max().unwrap();
		sum += max - min;
	}
	sum.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "5 1 9 5
7 5 3
2 4 6 8"
			.to_string();
		let result = invoke(&input);
		assert_eq!(result, "18");
	}
}
