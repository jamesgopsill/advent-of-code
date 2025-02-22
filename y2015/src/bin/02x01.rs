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
		let surface_area = present.surface_area();
		let smallest_area = present.areas.iter().min().unwrap();
		sum += surface_area + smallest_area
	}
	sum.to_string()
}

struct Present {
	areas: [u32; 3],
}

impl Present {
	fn new(s: &str) -> Self {
		let mut items = s.split("x");
		let mut sides: [u32; 3] = [0; 3];
		for side in &mut sides {
			*side = items.next().unwrap().parse::<u32>().unwrap();
		}
		let areas = [
			sides[0] * sides[1],
			sides[1] * sides[2],
			sides[2] * sides[0],
		];
		Self { areas }
	}

	fn surface_area(&self) -> u32 {
		self.areas[0] * 2 + self.areas[1] * 2 + self.areas[2] * 2
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("2x3x4");
		assert_eq!(result, "58");
	}

	#[test]
	fn test_b() {
		let result = invoke("1x1x10");
		assert_eq!(result, "43");
	}
}
