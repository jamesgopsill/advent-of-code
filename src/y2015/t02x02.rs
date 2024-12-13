pub fn invoke(input: &String) -> String {
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
		for i in 0..3 {
			sides[i] = items.next().unwrap().parse::<u32>().unwrap();
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
		let input = "2x3x4".to_string();
		let result = invoke(&input);
		assert_eq!(result, "34");
	}

	#[test]
	fn test_b() {
		let input = "1x1x10".to_string();
		let result = invoke(&input);
		assert_eq!(result, "14");
	}
}
