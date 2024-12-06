pub fn invoke(input: String) -> u32 {
	let mut sum: u32 = 0;
	for line in input.lines() {
		let present = Present::new(line);
		let surface_area = present.surface_area();
		let smallest_area = present.areas.iter().min().unwrap();
		sum += surface_area + smallest_area
	}
	sum
}

struct Present {
	areas: [u32; 3],
}

impl Present {
	fn new(s: &str) -> Self {
		let mut items = s.split("x");
		let mut sides: [u32; 3] = [0; 3];
		for i in 0..3 {
			sides[i] = items.next().unwrap().parse::<u32>().unwrap();
		}
		let areas = [
			sides[0] * sides[1],
			sides[1] * sides[2],
			sides[2] * sides[0],
		];
		Self { areas }
	}

	fn surface_area(&self) -> u32 {
		return self.areas[0] * 2 + self.areas[1] * 2 + self.areas[2] * 2;
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "2x3x4".to_string();
		let result = invoke(input);
		assert_eq!(result, 58);
	}

	#[test]
	fn test_b() {
		let input = "1x1x10".to_string();
		let result = invoke(input);
		assert_eq!(result, 43);
	}
}
