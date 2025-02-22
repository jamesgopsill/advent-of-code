use std::collections::HashMap;

use regex::Regex;

use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/16.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut tape: HashMap<&str, u32> = HashMap::new();
	tape.insert("children", 3);
	tape.insert("cats", 7);
	tape.insert("samoyeds", 2);
	tape.insert("pomeranians", 3);
	tape.insert("akitas", 0);
	tape.insert("vizslas", 0);
	tape.insert("goldfish", 5);
	tape.insert("trees", 3);
	tape.insert("cars", 2);
	tape.insert("perfumes", 1);

	let re = Regex::new(r"([a-z]+):\s(\d+)").unwrap();
	let mut aunt: u32 = 0;
	for (i, line) in input.lines().enumerate() {
		let mut flag = true;
		let caps = re.captures_iter(line);
		for c in caps {
			let item = c.get(1).unwrap().as_str();
			let val = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
			let tape_val = tape.get(item).unwrap();
			match item {
				"cats" | "trees" => {
					if val <= *tape_val {
						flag = false;
						break;
					}
				}
				"pomeranians" | "goldfish" => {
					if val >= *tape_val {
						flag = false;
						break;
					}
				}
				_ => {
					if val != *tape_val {
						flag = false;
						break;
					}
				}
			}
		}
		if flag {
			aunt = i as u32 + 1;
			break;
		}
	}
	aunt.to_string()
}
