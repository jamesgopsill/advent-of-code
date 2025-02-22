use std::collections::HashMap;
use std::fs;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/22.txt").unwrap();
	let out = invoke(&input, 0);
	println!("{}", out);
}

fn invoke(
	input: &str,
	n: u32,
) -> String {
	let mut cache: HashMap<u64, u64> = HashMap::new();
	let mut ans: u64 = 0;
	for line in input.lines() {
		println!("{}", line);
		let mut secret_number = line.parse::<u64>().unwrap();
		for _i in 0..n {
			let previous_number = secret_number;
			// Cache
			if let Some(val) = cache.get(&previous_number) {
				//println!("Using cache");
				secret_number = *val;
				continue;
			}
			// Step One
			let a = secret_number * 64;
			secret_number ^= a; //= secret_number ^ a;
			secret_number %= 16_777_216; // = secret_number % 16_777_216;
								// Step Two
			let b = secret_number / 32;
			secret_number ^= b; //= secret_number ^ b;
			secret_number %= 16_777_216; //= secret_number % 16_777_216;
								// Step Three
			let c = secret_number * 2048;
			secret_number ^= c; //= secret_number ^ c;
			secret_number %= 16_777_216; //= secret_number % 16_777_216;
								// Add to cache
			cache.insert(previous_number, secret_number);
			//println!("{}: {} -> {}", i + 1, previous_number, secret_number);
		}
		ans += secret_number;
	}
	println!("---");
	ans.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "123";
		let result = invoke(input, 10);
		assert_eq!(result, "5908254");
	}

	#[test]
	fn test_b() {
		let input = "1
10
100
2024";
		let result = invoke(input, 2_000);
		assert_eq!(result, "37327623");
	}
}
