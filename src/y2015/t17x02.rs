use itertools::Itertools;

pub fn invoke(
	input: &String,
	liters: u32,
) -> String {
	let mut minimum_number: usize = 9_999;
	let mut variations: u32 = 0;
	let mut containers: Vec<u32> = vec![];
	for line in input.lines() {
		containers.push(line.parse::<u32>().unwrap());
	}
	for c in containers.iter().powerset() {
		let n = c.len();
		let mut sum: u32 = 0;
		for v in c {
			sum += *v;
		}
		if sum == liters {
			if n < minimum_number {
				minimum_number = n;
				variations = 1;
			} else if n == minimum_number {
				variations += 1;
			}
		}
	}
	variations.to_string()
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
		let result = invoke(&input, 25);
		assert_eq!(result, "3");
	}
}
