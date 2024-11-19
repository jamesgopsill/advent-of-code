use itertools::Itertools;

pub fn invoke(
	input: String,
	liters: u32,
) -> u32 {
	let mut valid_combinations: u32 = 0;
	let mut containers: Vec<u32> = vec![];
	for line in input.lines() {
		containers.push(line.parse::<u32>().unwrap());
	}
	for c in containers.iter().powerset() {
		let mut sum: u32 = 0;
		for v in c {
			sum += *v;
		}
		if sum == liters {
			valid_combinations += 1;
		}
	}
	valid_combinations
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "20
15
10
5
5"
		.to_string();
		let result = invoke(input, 25);
		assert_eq!(result, 4);
	}
}
