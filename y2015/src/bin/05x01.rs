use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/05.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut nice_count = 0;
	for line in input.lines() {
		if is_nice(line) {
			nice_count += 1;
		}
	}
	nice_count.to_string()
}

fn is_nice(line: &str) -> bool {
	let naughty_patterns = ["ab", "cd", "pq", "xy"];
	for pattern in naughty_patterns {
		if line.contains(pattern) {
			//println!("Naughty naughty_pattern");
			return false;
		}
	}

	let vowels = ['a', 'e', 'i', 'o', 'u'];
	let mut count = 0;
	for vowel in vowels {
		for c in line.chars() {
			if vowel == c {
				count += 1;
			}
		}
	}
	if count < 3 {
		//println!("Naughty no three or more vowels");
		return false;
	}

	let mut double_char = false;
	for c in 'a'..='z' {
		let double = format!("{}{}", c, c);
		if line.contains(double.as_str()) {
			double_char = true;
			break;
		}
	}
	if !double_char {
		//println!("Naughty No Double Char");
		return false;
	}

	true
}

#[cfg(test)]
mod tests {
	use super::is_nice;

	#[test]
	fn test_a() {
		let result = is_nice("ugknbfddgicrmopn");
		assert!(result);
	}

	#[test]
	fn test_b() {
		let result = is_nice("aaa");
		assert!(result);
	}

	#[test]
	fn test_c() {
		let result = is_nice("jchzalrnumimnmhp");
		assert!(!result);
	}

	#[test]
	fn test_d() {
		let result = is_nice("haegwjzuvuyypxyu");
		assert!(!result);
	}

	#[test]
	fn test_e() {
		let result = is_nice("dvszwmarrgswjxmb");
		assert!(!result);
	}
}
