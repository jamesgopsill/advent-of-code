use std::collections::HashMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

//use std::fs;
//use utils::bench;

fn main() {
	todo!();
	//let input = fs::read_to_string("puzzle_data/2024/22.txt").unwrap();
	//let out = invoke(&input);
	//println!("{}", out);
	//bench(invoke, &input);
}

fn invoke(
	input: &str,
	n: u32,
) -> String {
	let mut cache: HashMap<u64, u64> = HashMap::new();
	let mut price_lists: Vec<Vec<(i8, i8)>> = vec![];
	for line in input.lines() {
		println!("{}", line);
		let mut secret_number = line.parse::<u64>().unwrap();
		let mut price_list: Vec<(i8, i8)> = vec![];
		for _i in 0..n {
			let previous_number = secret_number.clone();
			// Cache
			if let Some(val) = cache.get(&previous_number) {
				//println!("Using cache");
				secret_number = *val;
				let digit = last_digit(secret_number.to_string());
				price_list.push((digit, 0));
				continue;
			}
			// Step One
			let a = secret_number * 64;
			secret_number = secret_number ^ a;
			secret_number = secret_number % 16_777_216;
			// Step Two
			let b = secret_number / 32;
			secret_number = secret_number ^ b;
			secret_number = secret_number % 16_777_216;
			// Step Three
			let c = secret_number * 2048;
			secret_number = secret_number ^ c;
			secret_number = secret_number % 16_777_216;
			// get the last digit
			let digit = last_digit(secret_number.to_string());
			price_list.push((digit, 0));
			// Add to cache
			cache.insert(previous_number, secret_number);
			//println!("{}: {} -> {}", i + 1, previous_number, secret_number);
		}
		price_lists.push(price_list);
	}
	println!("-- Price Lists Gathered --");

	for price_list in price_lists.iter_mut() {
		for i in 0..price_list.len() - 1 {
			let diff = price_list[i + 1].0 - price_list[i].0;
			price_list[i + 1].1 = diff;
		}
		//println!("{:?}", price_list);
	}

	println!("-- Diff Done --");

	let mut combinations: Vec<(i8, i8, i8, i8)> = vec![];
	for i in -9..9 as i8 {
		for j in -9..9 as i8 {
			for k in -9..9 as i8 {
				for l in -9..9 as i8 {
					combinations.push((i, j, k, l));
				}
			}
		}
	}

	println!("-- Combinations Set --");

	// Brute force for the win.
	let haul = combinations
		.par_iter()
		.map(|(i, j, k, l)| {
			println!("{} {} {} {}", i, j, k, l);
			let mut bananas: u64 = 0;
			for price_list in price_lists.iter() {
				for win in price_list.windows(4) {
					// Buy some bananas on the first occurrence.
					if win[0].1 == *i && win[1].1 == *j && win[2].1 == *k && win[3].1 == *l {
						//println!("Banana price found: {}", win[3].1);
						bananas += win[3].0 as u64;
						break;
					}
				}
			}
			bananas
		})
		.max()
		.unwrap();
	println!("Top Haul: {}", haul);
	haul.to_string()
}

fn last_digit(s: String) -> i8 {
	let mut digits = s.chars().filter_map(|c| c.to_digit(10));
	digits.next_back().unwrap() as i8
}

#[cfg(test)]
mod tests_22x02 {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "1
2
3
2024";
		let result = invoke(&input, 2_000);
		assert_eq!(result, "23");
	}
}
