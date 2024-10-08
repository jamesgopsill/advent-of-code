pub fn invoke(
	input: String,
	debug: bool,
) -> u32 {
	let mut out = 0;
	let mut count: u32;
	let cards = input.lines();
	for card in cards {
		let mut card_numbers = card.split(":").last().unwrap().split("|");
		let winning_numbers: Vec<i32> = card_numbers
			.next()
			.unwrap()
			.trim()
			.split_whitespace()
			.map(|f| f.parse::<i32>().unwrap())
			.collect();
		let my_numbers: Vec<i32> = card_numbers
			.next()
			.unwrap()
			.trim()
			.split_whitespace()
			.map(|f| f.parse::<i32>().unwrap())
			.collect();
		count = 0;
		for wn in &winning_numbers {
			if my_numbers.contains(wn) {
				count += 1;
			}
		}
		if debug {
			println!("---");
			dbg!(winning_numbers, my_numbers, count);
		}
		if count == 1 {
			out += 1;
			continue;
		}
		if count > 1 {
			out += 2_u32.pow(count as u32 - 1);
			continue;
		}
	}
	out
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/04x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input, false);
		assert_eq!(result, 13);
	}
}
