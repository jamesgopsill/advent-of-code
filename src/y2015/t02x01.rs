pub fn invoke(
	input: String,
	_debug: bool,
) -> u32 {
	let mut wrapping_paper = 0;
	let dimensions: Vec<&str> = input.lines().collect();
	for d in dimensions {
		let present = Present::new(d);
		wrapping_paper += present.surface_area + present.smallest_face;
	}
	wrapping_paper
}

struct Present {
	_length: u32,
	_width: u32,
	_height: u32,
	_face_areas: [u32; 3],
	surface_area: u32,
	smallest_face: u32,
}

impl Present {
	fn new(dimensions: &str) -> Self {
		let mut dimensions: Vec<&str> = dimensions.split("x").collect();
		let length = dimensions.pop().unwrap().parse().unwrap();
		let width = dimensions.pop().unwrap().parse().unwrap();
		let height = dimensions.pop().unwrap().parse().unwrap();
		let face_areas: [u32; 3] = [length * width, width * height, length * height];
		let mut surface_area = 0;
		let mut smallest_face = face_areas[0];
		for area in face_areas {
			surface_area += area * 2;
			if area < smallest_face {
				smallest_face = area;
			}
		}
		Self {
			_length: length,
			_width: width,
			_height: height,
			_face_areas: face_areas,
			surface_area,
			smallest_face,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("2x3x4".to_string(), true);
		assert_eq!(result, 58);
	}

	#[test]
	fn test_b() {
		let result = invoke("1x1x10".to_string(), true);
		assert_eq!(result, 43);
	}
}
