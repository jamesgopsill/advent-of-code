use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/11.txt").unwrap();
	let out = invoke(&input, 25);
	println!("{}", out);
}

fn invoke(
	input: &str,
	blink: u64,
) -> String {
	let mut stones: Vec<u64> = input
		.split_whitespace()
		.map(|v| v.parse::<u64>().unwrap())
		.collect();

	for _ in 0..blink {
		let mut new_stones: Vec<u64> = vec![];
		for stone in stones.iter() {
			if *stone == 0 {
				new_stones.push(1);
				continue;
			}
			let s = format!("{}", stone);
			if s.len() % 2 == 0 {
				let (left, right) = s.split_at(s.len() / 2);
				new_stones.push(left.parse::<u64>().unwrap());
				new_stones.push(right.parse::<u64>().unwrap());
				continue;
			}
			new_stones.push(stone * 2024);
		}
		stones = new_stones;
	}

	stones.len().to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "0 1 10 99 999";
		let result = invoke(input, 1);
		assert_eq!(result, "7");
	}

	#[test]
	fn test_b() {
		let input = "125 17";
		let result = invoke(input, 6);
		assert_eq!(result, "22");
	}
}
