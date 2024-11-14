pub fn invoke(
	input: String,
	_debug: bool,
) -> u32 {
	let mut ribbon = 0;
	let dimensions: Vec<&str> = input.lines().collect();
	for d in dimensions {
		let present = Present::new(d);
		ribbon += present.smallest_perimeter + present.volume;
	}
	ribbon
}

struct Present {
	smallest_perimeter: u32,
	volume: u32,
}

impl Present {
	fn new(dimensions: &str) -> Self {
		let mut dimensions: Vec<&str> = dimensions.split("x").collect();
		let length: u32 = dimensions.pop().unwrap().parse().unwrap();
		let width: u32 = dimensions.pop().unwrap().parse().unwrap();
		let height: u32 = dimensions.pop().unwrap().parse().unwrap();
		let volume = length * width * height;
		let perimeters: [u32; 3] = [
			2 * length + 2 * width,
			2 * width + 2 * height,
			2 * length + 2 * height,
		];
		let smallest = *perimeters.iter().min().unwrap();
		Self {
			smallest_perimeter: smallest,
			volume,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("2x3x4".to_string(), true);
		assert_eq!(result, 34);
	}

	#[test]
	fn test_b() {
		let result = invoke("1x1x10".to_string(), true);
		assert_eq!(result, 14);
	}
}
