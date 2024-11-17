use itertools::Itertools;
use std::collections::HashMap;

use regex::Regex;

pub fn invoke(input: String) -> i32 {
	let input = input.trim();
	// let guests: Vec<String> = vec![];
	let re = Regex::new(r"(\w+)[\w\s]+(gain|lose)\s(\d+)[\w\s]+([A-Z]\w+)").unwrap();
	let mut relationships: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
	relationships.insert("Santa", HashMap::new());
	for capture in re.captures_iter(input) {
		let guest_a = capture.get(1).unwrap().as_str();
		if relationships.get(guest_a).is_none() {
			let mut map = HashMap::new();
			map.insert("Santa", 0);
			relationships.insert(guest_a, map);
			let map = relationships.get_mut("Santa").unwrap();
			map.insert(guest_a, 0);
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
	max_happiness
}
