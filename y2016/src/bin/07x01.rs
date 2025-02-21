use itertools::Itertools;
use regex::Regex;
use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2016/07.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut pattern = "".to_string();
	for p in ('a'..='z').permutations(2) {
		pattern.push(p[0]);
		pattern.push(p[1]);
		pattern.push(p[1]);
		pattern.push(p[0]);
		pattern.push('|');
	}
	pattern.pop();
	//println!("{}", pattern);
	let tls_re = Regex::new(pattern.as_str()).unwrap();
	let brackets_re = Regex::new(r"\[\w+\]").unwrap();
	let mut ans: u32 = 0;
	for line in input.lines() {
		let count_one = tls_re.find_iter(line).count();
		let line = brackets_re.replace_all(line, "-").to_string();
		//println!("{}", line);
		let count_two = tls_re.find_iter(line.as_str()).count();
		// if pattern only exists outside the brackets.
		if count_two > 0 && count_one == count_two {
			ans += 1;
		}
	}
	ans.to_string()
}

#[cfg(test)]
mod tests_0701 {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn";
		let result = invoke(input);
		assert_eq!(result, "2");
	}
}
