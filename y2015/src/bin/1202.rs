use regex::Regex;
use std::fs;
use std::sync::LazyLock;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/12.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

// TODO: arrays red catching me out I think.
fn invoke(input: &str) -> String {
	//let mut sum: i32 = 0;
	let curlies_re = Regex::new(r#"\{[\w\"\[\]:,-]+\}"#).unwrap();
	let squares_re = Regex::new(r#"\[[\w\"\[\],-]+\]"#).unwrap();

	let mut json = input.to_string();
	let max_iter = 100;

	for _i in 0..max_iter {
		// Check if there are any curly braces at the lowest level
		// i.e., no inner curlies.
		let curlies_exist = curlies_re.is_match(json.as_str());
		if curlies_exist {
			let mut curly_replacements: Vec<(String, String)> = vec![];
			// for each lowest level curly
			for curly in curlies_re.captures_iter(json.as_str()) {
				// taken it as owned so we can modify it later
				let curly_cap = curly.get(0).unwrap().as_str().to_string();
				// Will need a copy to change any [] in the object
				let mut modded_curly = curly_cap.clone();
				let mut square_replacements: Vec<(String, String)> = vec![];
				for square in squares_re.captures_iter(curly_cap.as_str()) {
					let square = square.get(0).unwrap().as_str().to_string();
					// Note. no need to worry about RED in arrays so
					// lets find them and change them.
					let replaced_square = square.replace("red", "blue");
					square_replacements.push((square, replaced_square));
				}
				for (pat, to) in square_replacements {
					modded_curly = modded_curly.replacen(pat.as_str(), to.as_str(), 1);
				}
				// Now check if the curly has a red attribute
				let value = check_red(&modded_curly);
				// println!("{} {}", modded_curly, value);
				curly_replacements.push((curly_cap, value));
			}
			for (pat, to) in curly_replacements {
				//println!("{} {}", pat, to);
				json = json.replacen(pat.as_str(), to.as_str(), 1);
			}
		} else {
			break;
		}
	}
	println!("Reduced Json: {}", json);
	json.trim().to_string()
}

static RED_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"red").unwrap());
static DIGITS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[-\d]+").unwrap());

fn check_red(s: &str) -> String {
	if RED_RE.is_match(s) {
		"0".to_string()
	} else {
		let digits = DIGITS_RE.captures_iter(s);
		let mut sub_sum: i32 = 0;
		for digit in digits {
			let d = digit.get(0).unwrap().as_str().parse::<i32>().unwrap();
			sub_sum += d;
		}
		sub_sum.to_string()
	}
}

/*
#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("[1,2,3]".to_string());
		assert_eq!(result, 6);
	}

	#[test]
	fn test_b() {
		let result = invoke(r#"{"a":2,"b":4}"#.to_string());
		assert_eq!(result, 6);
	}

	#[test]
	fn test_c() {
		let result = invoke(r#"{"a":[-1,1]}"#.to_string());
		assert_eq!(result, 0);
	}
}
*/
