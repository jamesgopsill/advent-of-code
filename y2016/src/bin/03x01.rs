use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2016/03.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
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
	valid.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("5 10 25");
		assert_eq!(result, "0");
	}
}
