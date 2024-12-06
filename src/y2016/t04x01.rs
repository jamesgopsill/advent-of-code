use regex::Regex;

pub fn invoke(input: String) -> u32 {
	let re = Regex::new(r"([a-z\-]+)(\d+)\[([a-z]+)").unwrap();
	let mut sum: u32 = 0;
	for line in input.lines() {
		let cap = re.captures(line).unwrap();
		let code = cap.get(1).unwrap().as_str();
		let number = cap.get(2).unwrap().as_str();
		let checksum = cap.get(3).unwrap().as_str();
		//println!("{} {} {}", code, number, checksum);
		let mut letters: Vec<(char, u32)> = vec![];
		for c in 'a'..='z' {
			let mut count = 0;
			for l in code.chars() {
				if l == c {
					count += 1;
				}
			}
			letters.push((c, count));
		}
		letters.sort_by(|a, b| b.1.cmp(&a.1));
		//println!("{:?}", letters);
		let check = format!(
			"{}{}{}{}{}",
			letters[0].0, letters[1].0, letters[2].0, letters[3].0, letters[4].0
		);
		//println!("{}", check);
		if checksum == check.as_str() {
			sum += number.parse::<u32>().unwrap();
		}
	}
	sum
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]"
			.to_string();
		let result = invoke(input);
		assert_eq!(result, 1514);
	}
}
