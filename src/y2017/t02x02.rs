use itertools::Itertools;

pub fn invoke(input: &String) -> String {
	let mut sum: u32 = 0;
	for line in input.lines() {
		let items = line.split_whitespace().collect::<Vec<_>>();
		let values = items
			.iter()
			.map(|v| v.parse::<u32>().unwrap())
			.collect::<Vec<_>>();

		// Iterate through each pair of values
		// in the vec and check if they can divide
		for c in values.iter().combinations(2) {
			if c[0] % c[1] == 0 {
				sum += c[0] / c[1];
				break;
			}
			if c[1] % c[0] == 0 {
				sum += c[1] / c[0];
				break;
			}
		}
	}
	sum.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "5 9 2 8
9 4 7 3
3 8 6 5"
			.to_string();
		let result = invoke(&input);
		assert_eq!(result, "9");
	}
}
