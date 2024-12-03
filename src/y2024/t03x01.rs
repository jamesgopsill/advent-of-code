use regex::Regex;

pub fn invoke(input: &String) -> u32 {
	let re = Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)").unwrap();
	let caps = re.captures_iter(input.as_str());
	let mut sum: u32 = 0;
	for cap in caps {
		let a = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
		let b = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
		// println!("{} * {}", a, b);
		sum += a * b;
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			&"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
		);
		assert_eq!(result, 161);
	}
}
