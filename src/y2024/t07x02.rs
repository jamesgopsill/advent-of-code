pub fn invoke(input: &String) -> String {
	let mut ans: u64 = 0;
	for line in input.lines() {
		let (test, numbers) = line.split_once(":").unwrap();
		//println!("{}", test);
		let test = test.parse::<u64>().unwrap();
		let numbers = numbers.split_whitespace().collect::<Vec<_>>();
		let numbers = numbers
			.iter()
			.map(|v| v.parse::<u64>().unwrap())
			.collect::<Vec<u64>>();
		let initial = vec![numbers[0]];
		let mut next = numbers[1..].to_vec();
		// reverse so we can pop.
		next.reverse();
		let values = calculate(test, initial, next);
		if values.contains(&test) {
			// println!("Valid: {}", line);
			ans += test;
		}
	}
	ans.to_string()
}

fn calculate(
	test: u64,
	past: Vec<u64>,
	mut numbers: Vec<u64>,
) -> Vec<u64> {
	let number = numbers.pop();
	if number.is_none() {
		return past;
	}
	let number = number.unwrap();
	let mut next: Vec<u64> = vec![];
	for p in past {
		// Do not continue computing values beyond
		// the test value.
		if p > test {
			continue;
		}
		// println!("{} {}", p, number);
		// Ignore values that overflow.
		let new_value = p.checked_mul(number);
		if new_value.is_some() {
			next.push(new_value.unwrap());
		}
		let new_value = p.checked_add(number);
		if new_value.is_some() {
			next.push(new_value.unwrap());
		}
		// concatenate
		let new_value = concat(p, number) as u64;
		next.push(new_value);
		// Old 'slow' version
		/*
		let new_value = format!("{}{}", p, number);
		let new_value = new_value.parse::<u64>();
		if new_value.is_ok() {
			next.push(new_value.unwrap());
		}
		*/
	}
	calculate(test, next, numbers)
}

fn concat(
	a: u64,
	b: u64,
) -> u128 {
	a as u128 * 10u128.pow(b.ilog10() + 1) + b as u128
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			&"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"
			.to_string(),
		);
		assert_eq!(result, "11387");
	}
}
