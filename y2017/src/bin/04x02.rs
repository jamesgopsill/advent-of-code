use std::{collections::HashSet, fs};
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2017/04.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let passphrases = input.lines();
	let mut count = 0;
	for phrase in passphrases {
		if is_valid(phrase) {
			count += 1;
		}
	}
	count.to_string()
}

fn sorted_chars(s: &str) -> Vec<char> {
	let mut chars: Vec<char> = s.chars().collect();
	chars.sort();
	chars
}

fn is_valid(s: &str) -> bool {
	let words: Vec<Vec<char>> = s.split_whitespace().map(sorted_chars).collect();
	let set: HashSet<Vec<char>> = HashSet::from_iter(words.iter().cloned());
	words.len() == set.len()
}

#[cfg(test)]
mod tests {
	use crate::is_valid;

	#[test]
	fn test_a() {
		let input = "abcde fghij";
		let valid = is_valid(input);
		assert!(valid);
	}

	#[test]
	fn test_b() {
		let input = "abcde xyz ecdab";
		let valid = is_valid(input);
		assert!(!valid);
	}

	#[test]
	fn test_c() {
		let input = "a ab abc abd abf abj";
		let valid = is_valid(input);
		assert!(valid);
	}

	#[test]
	fn test_d() {
		let input = "iiii oiii ooii oooi oooo";
		let valid = is_valid(input);
		assert!(valid);
	}

	#[test]
	fn test_e() {
		let input = "oiii ioii iioi iiio";
		let valid = is_valid(input);
		assert!(!valid);
	}
}
