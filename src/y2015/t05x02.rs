use regex::Regex;

pub fn invoke(input: String) -> u32 {
	let mut nice_count = 0;

	let pairs = generate_pairs();
	let regs = generate_regs();
	for line in input.lines() {
		let result = is_nice(line, &pairs, &regs);
		if result {
			nice_count += 1;
		}
	}

	nice_count
}

fn generate_pairs() -> Vec<String> {
	// Create the possible pairings
	let mut pairs: Vec<String> = vec![];
	for a in b'a'..=b'z' {
		let a = a as char;
		for b in b'a'..=b'z' {
			let b = b as char;
			let p = format!("{}{}", a, b);
			pairs.push(p);
		}
	}
	pairs
}

fn generate_regs() -> Vec<Regex> {
	let mut regs = vec![];
	for a in b'a'..=b'z' {
		let a = a as char;
		let p = format!("{}[a-z]{}", a, a);
		let re = Regex::new(p.as_str()).unwrap();
		regs.push(re);
	}
	regs
}

fn is_nice(
	line: &str,
	pairs: &Vec<String>,
	regs: &Vec<Regex>,
) -> bool {
	let mut contains_pair = false;
	for p in pairs {
		let m = line.matches(p);
		if m.count() > 1 {
			contains_pair = true;
			break;
		}
	}
	if !contains_pair {
		return false;
	}
	let mut repeats = false;
	for re in regs {
		let first = re.find(&line);
		if first.is_some() {
			repeats = true;
			break;
		}
	}
	if !repeats {
		return false;
	}
	return true;
}

#[cfg(test)]
mod tests {
	use crate::y2015::t05x02::generate_regs;

	use super::{generate_pairs, is_nice};

	#[test]
	fn test_a() {
		let pairs = generate_pairs();
		let regs = generate_regs();
		let result = is_nice("qjhvhtzxzqqjkmpb", &pairs, &regs);
		assert_eq!(result, true);
	}

	#[test]
	fn test_b() {
		let pairs = generate_pairs();
		let regs = generate_regs();
		let result = is_nice("xxyxx", &pairs, &regs);
		assert_eq!(result, true);
	}

	#[test]
	fn test_c() {
		let pairs = generate_pairs();
		let regs = generate_regs();
		let result = is_nice("uurcxstgmygtbstg", &pairs, &regs);
		assert_eq!(result, false);
	}

	#[test]
	fn test_d() {
		let pairs = generate_pairs();
		let regs = generate_regs();
		let result = is_nice("ieodomkazucvgmuy", &pairs, &regs);
		assert_eq!(result, false);
	}

	#[test]
	fn test_e() {
		let pairs = generate_pairs();
		let regs = generate_regs();
		let result = is_nice("aaa", &pairs, &regs);
		assert_eq!(result, false);
	}
}
