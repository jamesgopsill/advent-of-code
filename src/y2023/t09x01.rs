pub fn invoke(
	input: String,
	debug: bool,
) -> i32 {
	let mut sum: i32 = 0;
	let lines = input.lines();
	for line in lines {
		let mut current_values: Vec<i32> = line
			.split_whitespace()
			.map(|v| v.parse::<i32>().unwrap())
			.collect();
		let mut line_sum = current_values.last().unwrap().clone();
		let mut next_values: Vec<i32> = vec![];
		if debug {
			dbg!(&current_values);
		}
		loop {
			next_values.clear();
			for win in current_values.windows(2) {
				next_values.push(win[1] - win[0]);
			}
			if debug {
				dbg!(&next_values);
			}
			let last_value = next_values.last().unwrap();
			if debug {
				dbg!(last_value);
			}
			line_sum += last_value;
			let s: i32 = next_values.iter().sum();
			if s == 0_i32 {
				break;
			}
			current_values.clear();
			current_values.append(&mut next_values);
		}
		if debug {
			dbg!(line_sum);
		}
		sum += line_sum;
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/2023/09x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input, true);
		assert_eq!(result, 114);
	}
}
