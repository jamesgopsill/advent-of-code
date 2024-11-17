use std::sync::LazyLock;

use regex::Regex;

pub fn invoke(
	input: String,
	n: u32,
) -> String {
	// reverse the string to ease the iteration
	let mut password = input.trim().to_string();
	// Placing an arbitrary limit to exit
	let mut found: u32 = 0;
	let max_iter = 1_000_000;
	println!("Finding {} passwords", n);
	for i in 0..max_iter {
		// println!("{}", password);
		let valid = is_valid_password(password.as_str());
		if valid {
			println!("Next password: {} {}", i, password);
			found += 1;
			if found == n {
				break;
			}
		}
		// Else increment letters (working backwards)
		// logic is wrong. Only lokk at the preceding char if one flips over.
		let mut idx = password.len() - 1;
		loop {
			let next_char = increment_char(&mut password, idx);
			if next_char {
				idx -= 1;
			} else {
				break;
			}
		}
	}
	password
}

fn increment_char(
	password: &mut String,
	idx: usize,
) -> bool {
	let c = password.chars().nth(idx).unwrap();
	if c == 'z' {
		password.replace_range(idx..(idx + 1), "a");
		return true;
	} else {
		let mut byte = c as u8;
		byte += 1;
		let new_c = byte as char;
		let new_c = format!("{}", new_c);
		password.replace_range(idx..(idx + 1), new_c.as_str());
		return false;
	}
}

fn is_valid_password(s: &str) -> bool {
	let passed = contains_straight(s);
	if !passed {
		return passed;
	}
	let passed = does_not_contain_invalid_letters(s);
	if !passed {
		return passed;
	}
	contains_two_pairs(s)
}

static STRAIGHT_RE: LazyLock<Regex> = LazyLock::new(|| {
	println!("Initializing Straight Re");
	let mut pattern = "".to_string();
	for c in b'b'..=b'y' {
		pattern.push((c - 1) as char);
		pattern.push((c) as char);
		pattern.push((c + 1) as char);
		pattern.push('|');
	}
	pattern.pop();
	Regex::new(pattern.as_str()).unwrap()
});

fn contains_straight(s: &str) -> bool {
	let passed = STRAIGHT_RE.is_match(s);
	passed
}

fn does_not_contain_invalid_letters(s: &str) -> bool {
	let chars = ['o', 'i', 'l'];
	for c in chars {
		if s.contains(c) {
			return false;
		}
	}
	true
}

static PAIRS_RE: LazyLock<Regex> = LazyLock::new(|| {
	println!("Initializing Pairs Re");
	let mut pattern = "".to_string();
	for c in b'a'..=b'z' {
		pattern.push((c) as char);
		pattern.push((c) as char);
		pattern.push('|');
	}
	pattern.pop();
	Regex::new(pattern.as_str()).unwrap()
});

fn contains_two_pairs(s: &str) -> bool {
	let mut n: u32 = 0;
	for _ in PAIRS_RE.find_iter(s) {
		n += 1;
	}
	if n >= 2 {
		return true;
	}
	false
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_a() {
		let result = contains_straight("hijklmmn");
		assert_eq!(result, true);
	}

	#[test]
	fn test_b() {
		let result = contains_straight("abbceffg");
		assert_eq!(result, false);
	}

	#[test]
	fn test_c() {
		let result = does_not_contain_invalid_letters("hijklmmn");
		assert_eq!(result, false);
	}

	#[test]
	fn test_d() {
		let result = contains_two_pairs("abbceffg");
		assert_eq!(result, true);
	}

	#[test]
	fn test_e() {
		let result = contains_two_pairs("hijklmmn");
		assert_eq!(result, false);
	}
}

/*
for i in (0..password.len()).rev() {
	if i == password.len() - 1 {
		let c = password.chars().nth(i).unwrap();
		if c == 'z' {
			password.replace_range(i..(i + 1), "a");
		} else {
			let mut byte = c as u8;
			byte += 1;
			let new_c = byte as char;
			let new_c = format!("{}", new_c);
			password.replace_range(i..(i + 1), new_c.as_str());
		}
	} else {
		let preceding_char = password.chars().nth(i + 1).unwrap();
		if preceding_char == 'a' {
			let c = password.chars().nth(i).unwrap();
			if c == 'z' {
				password.replace_range(i..(i + 1), "a");
			}
			else {

				let mut byte = c as u8;
				byte += 1;
				let new_c = byte as char;
				let new_c = format!("{}", new_c);
				password.replace_range(i..(i + 1), new_c.as_str());
			}

		}
	}
}
*/
