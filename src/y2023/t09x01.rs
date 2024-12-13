pub fn invoke(input: String) -> i32 {
	let mut sum: i32 = 0;
	let lines = input.lines();
	for line in lines {
		let mut current_values: Vec<i32> = line
			.split_whitespace()
			.map(|v| v.parse::<i32>().unwrap())
			.collect();
		let mut line_sum = current_values.last().unwrap().clone();
		let mut next_values: Vec<i32> = vec![];

		// dbg!(&current_values);

		loop {
			next_values.clear();
			for win in current_values.windows(2) {
				next_values.push(win[1] - win[0]);
			}

			// dbg!(&next_values);

			let last_value = next_values.last().unwrap();

			//	dbg!(last_value);

			line_sum += last_value;
			let s: i32 = next_values.iter().sum();
			if s == 0_i32 {
				break;
			}
			current_values.clear();
			current_values.append(&mut next_values);
		}

		//	dbg!(line_sum);

		sum += line_sum;
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
			.to_string();
		let result = invoke(input);
		assert_eq!(result, 114);
	}
}
