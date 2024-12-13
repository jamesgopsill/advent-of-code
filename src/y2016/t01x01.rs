pub fn invoke(input: &String) -> String {
	let moves = input.split(",");
	let mut elf = Elf::new();
	for m in moves {
		elf.walk(m.trim());
	}
	println!("{} {}", elf.x, elf.y);
	elf.distance().to_string()
}

enum Direction {
	North,
	East,
	South,
	West,
}

struct Elf {
	facing: Direction,
	x: i32,
	y: i32,
}

impl Elf {
	fn new() -> Self {
		Self {
			facing: Direction::North,
			x: 0,
			y: 0,
		}
	}

	fn walk(
		&mut self,
		instruction: &str,
	) {
		let direction = &instruction[0..1];
		let distance = instruction[1..].parse::<i32>().unwrap();

		match self.facing {
			Direction::North => match direction {
				"R" => {
					self.facing = Direction::East;
					self.x += distance;
				}
				"L" => {
					self.facing = Direction::West;
					self.x -= distance;
				}
				_ => {}
			},
			Direction::West => match direction {
				"R" => {
					self.facing = Direction::North;
					self.y += distance;
				}
				"L" => {
					self.facing = Direction::South;
					self.y -= distance;
				}
				_ => {}
			},
			Direction::South => match direction {
				"R" => {
					self.facing = Direction::West;
					self.x -= distance;
				}
				"L" => {
					self.facing = Direction::East;
					self.x += distance;
				}
				_ => {}
			},
			Direction::East => match direction {
				"R" => {
					self.facing = Direction::South;
					self.y -= distance;
				}
				"L" => {
					self.facing = Direction::North;
					self.y += distance;
				}
				_ => {}
			},
		}
	}

	fn distance(&self) -> i32 {
		self.x.abs() + self.y.abs()
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(&"R2, L3".to_string());
		assert_eq!(result, "5");
	}

	#[test]
	fn test_b() {
		let result = invoke(&"R2, R2, R2".to_string());
		assert_eq!(result, "2");
	}

	#[test]
	fn test_c() {
		let result = invoke(&"R5, L5, R5, R3".to_string());
		assert_eq!(result, "12");
	}
}
