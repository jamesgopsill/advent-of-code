use std::collections::HashMap;

pub fn invoke(
	input: &String,
	blink: u64,
) -> u64 {
	// No Brute Force

	let starting_stones: Vec<u64> = input
		.split_whitespace()
		.into_iter()
		.map(|v| v.parse::<u64>().unwrap())
		.collect();

	let mut stone_counts: HashMap<u64, u64> = HashMap::new();

	for stone in starting_stones {
		if let Some(v) = stone_counts.get_mut(&stone) {
			*v += 1;
		} else {
			stone_counts.insert(stone, 1);
		}
	}

	// blinking
	for _ in 0..blink {
		let mut new_stone_counts: HashMap<u64, u64> = HashMap::new();
		for (stone_number, stone_count) in stone_counts.iter() {
			// Rule 1
			if *stone_number == 0 {
				let new_stone_number = 1;
				if let Some(v) = new_stone_counts.get_mut(&new_stone_number) {
					*v += *stone_count;
				} else {
					new_stone_counts.insert(new_stone_number, *stone_count);
				}
				continue;
			}
			// Rule 2
			let s = format!("{}", stone_number);
			if s.len() % 2 == 0 {
				let (left, right) = s.split_at(s.len() / 2);
				let left = left.parse::<u64>().unwrap();
				if let Some(v) = new_stone_counts.get_mut(&left) {
					*v += *stone_count;
				} else {
					new_stone_counts.insert(left, *stone_count);
				}
				let right = right.parse::<u64>().unwrap();
				if let Some(v) = new_stone_counts.get_mut(&right) {
					*v += *stone_count;
				} else {
					new_stone_counts.insert(right, *stone_count);
				}
				continue;
			}
			// Rule 3
			let new_stone_number = stone_number * 2024;
			if let Some(v) = new_stone_counts.get_mut(&new_stone_number) {
				*v += *stone_count;
			} else {
				new_stone_counts.insert(new_stone_number, *stone_count);
			}
		}
		stone_counts = new_stone_counts;
	}

	// sum counts
	let mut sum = 0;
	for (_, sc) in stone_counts.iter() {
		sum += sc;
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "0 1 10 99 999".to_string();
		let result = invoke(&input, 1);
		assert_eq!(result, 7);
	}

	#[test]
	fn test_b() {
		let input = "125 17".to_string();
		let result = invoke(&input, 6);
		assert_eq!(result, 22);
	}
}

/*
use rayon::prelude::*;

pub fn invoke(
	input: &String,
	blink: u64,
) -> u64 {
	let mut stones: Vec<String> = input
		.split_whitespace()
		.into_iter()
		.map(|v| v.to_string())
		.collect();

	for i in 0..blink {
		let new_stones: Vec<String> = stones
			.par_iter()
			.map(|stone| {
				let matches: Vec<_> = stone.matches("0").collect();
				// All zeros
				if matches.len() == stone.len() {
					return "1".to_string();
				}
				if stone.len() % 2 == 0 {
					let mut stones = stone.clone();
					stones.insert_str(stones.len() / 2, " ");
					return stones;
				}
				let number = stone.parse::<u64>().unwrap() * 2024;
				format!("{}", number)
			})
			.collect();
		let new_stones = new_stones.join(" ");
		stones = new_stones
			.split_whitespace()
			.into_iter()
			.map(|v| v.to_string())
			.collect();
		println!("{} {}", i, stones.len())
	}

	stones.len() as u64
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "0 1 10 99 999".to_string();
		let result = invoke(&input, 1);
		assert_eq!(result, 7);
	}

	#[test]
	fn test_b() {
		let input = "125 17".to_string();
		let result = invoke(&input, 6);
		assert_eq!(result, 22);
	}
}
*/
