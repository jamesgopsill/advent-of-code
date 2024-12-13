pub fn invoke(input: String) -> u32 {
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

		// println!("---");
		// dbg!(winning_numbers, my_numbers, count);

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

	#[test]
	fn test() {
		let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
			.to_string();
		let result = invoke(input);
		assert_eq!(result, 13);
	}
}
