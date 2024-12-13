pub fn invoke(input: String) -> u32 {
	let lines: Vec<&str> = input.lines().collect();
	let mut arrangements = 0;
	for line in lines {
		let parts: Vec<&str> = line.split(" ").collect();
		let damaged_map = parts.first().unwrap().to_string();
		// Identify where the question marks and hashes are
		let mut hash_indices: Vec<usize> = vec![];
		let mut hash_question_indices: Vec<usize> = vec![];
		for (i, c) in damaged_map.chars().enumerate() {
			if c == '#' {
				hash_indices.push(i);
				hash_question_indices.push(i);
			}
			if c == '?' {
				hash_question_indices.push(i);
			}
		}
		// Retrieve the spring pattern
		// And develop the list of possible connections.
		let spring_pattern: Vec<&str> = parts.last().unwrap().split(",").collect();
		let mut previous_combinations: Vec<String> = vec![];
		let mut new_combinations: Vec<String> = vec![];
		let empty_pattern = ".".repeat(damaged_map.len());
		previous_combinations.push(empty_pattern);
		for value in spring_pattern {
			let v: usize = value.parse().unwrap();
			let springs = "#".repeat(v);
			for combination in previous_combinations {
				let mut start = 0;
				let needle = combination.rfind("#");
				if needle.is_some() {
					start = needle.unwrap() + 2;
				}
				let end = combination.len() + 1 - v;
				for idx in start..end {
					let mut new_combination = combination.clone();
					new_combination.replace_range(idx..(idx + v), springs.as_str());
					new_combinations.push(new_combination);
				}
			}
			// println!("{:?}", new_combinations);
			previous_combinations = new_combinations.clone();
			new_combinations.clear();
		}
		// println!("{:?}", previous_combinations);
		// Now to do the validity checks
		let mut flag: bool;
		let mut row_arrangements: u32 = 0;
		for combination in previous_combinations {
			flag = true;
			// Check the hashes are covered
			for idx in &hash_indices {
				if combination.chars().nth(*idx).unwrap() != '#' {
					flag = false;
					break;
				}
			}
			// Check that positions are either hashes or questions marks.
			for (i, c) in combination.chars().enumerate() {
				if c == '#' && !hash_question_indices.contains(&i) {
					flag = false;
					break;
				}
			}
			// println!("{:?} {:?} {:?}", damaged_map, combination, flag);
			if flag == true {
				row_arrangements += 1;
			}
		}
		// println!("{:?}", row_arrangements);
		arrangements += row_arrangements
	}
	arrangements
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"
		.to_string();
		let result = invoke(input);
		assert_eq!(result, 21);
	}
}
