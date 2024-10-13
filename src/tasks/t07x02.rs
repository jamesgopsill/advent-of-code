use std::collections::HashMap;

const CARD_RANKS: [char; 13] = [
	'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug)]
struct Bet {
	hand: String,
	modified_hand: String,
	cards: Vec<usize>,
	bet: u32,
	hand_type: u32,
}

impl Bet {
	pub fn new(s: &str) -> Self {
		let elements: Vec<&str> = s.split_whitespace().collect();
		let hand = elements[0].to_string();
		let mut modified_hand = elements[0].to_string();
		let bet = elements[1].parse::<u32>().unwrap();

		// A map to help work out the hand type
		// Based on card occurrence.
		let mut map: HashMap<char, u32> = HashMap::new();
		for char in CARD_RANKS {
			map.insert(char, 0);
		}

		// Get the vec of cards
		let mut cards: Vec<usize> = vec![];
		for char in hand.chars() {
			let idx = CARD_RANKS.iter().position(|&r| r == char).unwrap();
			cards.push(idx);
			let count = map.get(&char).unwrap();
			map.insert(char, count + 1);
		}

		// Modifiction. Check for any J cards and convert them
		// into the card which has the highest occurrence then value.
		let n_jokers = *map.get(&'J').unwrap();
		if n_jokers > 0 {
			// Set J to 0 as we are going to 'change' its value
			// Also avoids the issue where J appears more than any
			// other hand.
			map.insert('J', 0);
			let mut occurrence: u32 = 0;
			let mut joker_should_be: char = '~';
			for (key, val) in map.iter() {
				if *val >= occurrence {
					joker_should_be = *key;
					occurrence = *val;
				}
			}
			map.insert(joker_should_be, occurrence + n_jokers);
			modified_hand = modified_hand.replace("J", format!("{}", joker_should_be).as_str());
		}
		println!("Modified Hand: {}", modified_hand);

		// Work out the hand type.
		let mut hand_type: u32 = 0;
		let mut twos = 0;
		let mut threes = 0;
		for (_, value) in map.into_iter() {
			if value == 5 {
				hand_type = 6;
				break;
			}
			if value == 4 {
				hand_type = 5;
				break;
			}
			if value == 3 {
				threes += 1;
				continue;
			}
			if value == 2 {
				twos += 1;
				continue;
			}
		}
		if threes == 1 && twos == 1 {
			hand_type = 4;
		} else if threes == 1 {
			hand_type = 3;
		} else if twos == 2 {
			hand_type = 2;
		} else if twos == 1 {
			hand_type = 1;
		}

		Self {
			hand,
			modified_hand,
			cards,
			bet,
			hand_type,
		}
	}
}

pub fn invoke(input: String) -> u32 {
	let mut sum: u32 = 0;
	let mut bets: Vec<Bet> = vec![];
	let lines = input.lines();
	for line in lines {
		let bet = Bet::new(line);
		bets.push(bet);
	}
	//println!("Unsorted");
	/*for bet in &bets {
		println!("{:?}", bet);
	}*/

	// Sort (in ascending order) by cards and their appearance
	bets.sort_by(|a, b| a.cards[4].cmp(&b.cards[4]));
	bets.sort_by(|a, b| a.cards[3].cmp(&b.cards[3]));
	bets.sort_by(|a, b| a.cards[2].cmp(&b.cards[2]));
	bets.sort_by(|a, b| a.cards[1].cmp(&b.cards[1]));
	bets.sort_by(|a, b| a.cards[0].cmp(&b.cards[0]));

	// Sort by the hand type.
	bets.sort_by(|a, b| a.hand_type.cmp(&b.hand_type));

	/*
	println!("Sorted");
	for bet in &bets {
		println!("{:?}", bet);
	}*/

	for (i, bet) in bets.iter().enumerate() {
		let winnings = (i + 1) as u32 * bet.bet;
		sum += winnings;
	}

	sum
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/07x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input);
		assert_eq!(result, 5905);
	}
}
