use std::mem::swap;

pub fn invoke(
	input: String,
	debug: bool,
) -> u32 {
	let mut card_map: Vec<Vec<usize>> = vec![];

	let mut current_pile: Vec<usize> = vec![];
	let mut new_pile: Vec<usize> = vec![];

	let mut winning_count: u32;
	let cards = input.lines();
	for (i, card) in cards.enumerate() {
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
		winning_count = 0;
		for wn in &winning_numbers {
			if my_numbers.contains(wn) {
				winning_count += 1;
			}
		}
		if debug {
			println!("---");
			dbg!(winning_numbers, my_numbers, winning_count);
		}
		current_pile.push(i);
		let map: Vec<usize> = (i + 1..i + winning_count as usize + 1).collect();
		card_map.push(map);
	}

	let mut count: u32 = 0;
	while current_pile.len() > 0 {
		for card in &current_pile {
			count += 1;
			let new_cards = &card_map[*card];
			new_pile.extend(new_cards);
		}
		swap(&mut current_pile, &mut new_pile);
		new_pile.clear();
	}

	return count;
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/04x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input, true);
		assert_eq!(result, 30);
	}
}
