use regex::Regex;
use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/03.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let re = Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)").unwrap();
	let caps = re.captures_iter(input);
	let mut sum: u32 = 0;
	for cap in caps {
		let a = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
		let b = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
		// println!("{} * {}", a, b);
		sum += a * b;
	}
	sum.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result =
			invoke("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
		assert_eq!(result, "161");
	}
}
