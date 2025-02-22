use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/13.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let input = input.trim();
	// let guests: Vec<String> = vec![];
	let re = Regex::new(r"(\w+)[\w\s]+(gain|lose)\s(\d+)[\w\s]+([A-Z]\w+)").unwrap();
	let mut relationships: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
	for capture in re.captures_iter(input) {
		let guest_a = capture.get(1).unwrap().as_str();
		if !relationships.contains_key(guest_a) {
			relationships.insert(guest_a, HashMap::new());
		}
		let direction = capture.get(2).unwrap().as_str();
		let mut score = capture.get(3).unwrap().as_str().parse::<i32>().unwrap();
		if direction == "lose" {
			score = -score;
		}
		let guest_b = capture.get(4).unwrap().as_str();
		let map = relationships.get_mut(guest_a).unwrap();
		map.insert(guest_b, score);
	}
	// Get the guest list
	let mut guests: Vec<&str> = vec![];
	for key in relationships.keys() {
		guests.push(*key);
	}
	let n_guests = guests.len();
	println!("Guests: {}", n_guests);

	// Calculate the scores for all the permutations
	let permutations = guests.into_iter().permutations(n_guests);
	let mut max_happiness: i32 = 0;
	for permutation in permutations {
		let mut happiness: i32 = 0;
		for i in 0..permutation.len() - 1 {
			let a = permutation[i];
			let b = permutation[i + 1];
			happiness += relationships.get(a).unwrap().get(b).unwrap();
			happiness += relationships.get(b).unwrap().get(a).unwrap();
		}
		let a = *permutation.first().unwrap();
		let b = *permutation.last().unwrap();
		happiness += relationships.get(a).unwrap().get(b).unwrap();
		happiness += relationships.get(b).unwrap().get(a).unwrap();
		if happiness > max_happiness {
			println!("{:?} {}", permutation, happiness);
			max_happiness = happiness;
		}
	}
	max_happiness.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			r"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
",
		);
		assert_eq!(result, "330");
	}
}
