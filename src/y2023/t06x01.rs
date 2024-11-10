use regex::Regex;

pub fn invoke(input: String) -> i32 {
	let mut sum: i32 = 1;
	let numbers_re = Regex::new(r"\d+").unwrap();
	let lines: Vec<&str> = input.lines().collect();
	let times: Vec<i32> = numbers_re
		.find_iter(lines[0])
		.map(|f| f.as_str().parse::<i32>().unwrap())
		.collect();
	let distances: Vec<i32> = numbers_re
		.find_iter(lines[1])
		.map(|f| f.as_str().parse::<i32>().unwrap())
		.collect();
	let n_races = times.len();
	let mut wins: i32;
	let mut dist: i32;
	for i in 0..n_races {
		let t = times[i];
		let d = distances[i];
		wins = 0;
		for j in 1..t {
			dist = j * (t - j);
			if dist > d {
				wins += 1;
			}
		}
		println!("{} {}", i, wins);
		sum *= wins;
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/2023/06x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input);
		assert_eq!(result, 288);
	}
}
