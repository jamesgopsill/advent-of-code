use std::{collections::HashSet, fs};
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2017/06.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn to_u32(v: &str) -> u32 {
	v.parse::<u32>().unwrap()
}

fn invoke(input: &str) -> String {
	let mut memory_banks: Vec<u32> = input.split_whitespace().map(to_u32).collect();
	let n_banks = memory_banks.len();
	let mut configurations: HashSet<Vec<u32>> = HashSet::new();
	let mut cycles = 0;
	configurations.insert(memory_banks.clone());

	loop {
		cycles += 1;
		// Find the highest bank
		let mut idx = 0;
		let mut size = memory_banks[0];
		for (i, bank) in memory_banks.iter().enumerate() {
			if *bank > size {
				idx = i;
				size = *bank;
			}
		}
		// Now re-distribute
		memory_banks[idx] = 0;
		while size > 0 {
			if idx == n_banks - 1 {
				idx = 0;
			} else {
				idx += 1;
			}
			memory_banks[idx] += 1;
			size -= 1;
		}
		// Check if we have seen the config before
		if configurations.contains(&memory_banks) {
			break;
		}
		configurations.insert(memory_banks.clone());
	}

	cycles.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "0 2 7 0";
		let result = invoke(input);
		assert_eq!(result, "5");
	}
}
