use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/02.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut sum: u32 = 0;
	for line in input.lines() {
		let present = Present::new(line);
		sum += present.volume + present.perimeters.iter().min().unwrap();
	}
	sum.to_string()
}

struct Present {
	perimeters: [u32; 3],
	volume: u32,
}

impl Present {
	fn new(s: &str) -> Self {
		let mut items = s.split("x");
		let mut sides: [u32; 3] = [0; 3];
		for side in sides.as_mut() {
			*side = items.next().unwrap().parse::<u32>().unwrap();
		}
		let volume = sides[0] * sides[1] * sides[2];
		let perimeters = [
			2 * sides[0] + 2 * sides[1],
			2 * sides[1] + 2 * sides[2],
			2 * sides[2] + 2 * sides[0],
		];
		Self { perimeters, volume }
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("2x3x4");
		assert_eq!(result, "34");
	}

	#[test]
	fn test_b() {
		let result = invoke("1x1x10");
		assert_eq!(result, "14");
	}
}
