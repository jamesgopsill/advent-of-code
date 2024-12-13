pub fn invoke(input: &String) -> String {
	let moves = input.split(",");
	let mut elf = Elf::new();
	let mut visited: Vec<String> = vec![];
	for m in moves {
		let instruction = m.trim();
		let direction = &instruction[0..1];
		let distance = instruction[1..].parse::<i32>().unwrap();

		elf.change_direction(direction);
		for _ in 0..distance {
			elf.step();
			let v = format!("{}_{}", elf.x, elf.y);
			if visited.contains(&v) {
				// visited twice
				println!("Visited {} twice", v);
				return elf.distance().to_string();
			}
			visited.push(v);
		}
	}
	println!("Did not visit a place twice!");
	return 0.to_string();
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

	fn change_direction(
		&mut self,
		direction: &str,
	) {
		match self.facing {
			Direction::North => match direction {
				"R" => self.facing = Direction::East,
				"L" => self.facing = Direction::West,
				_ => {}
			},
			Direction::West => match direction {
				"R" => self.facing = Direction::North,
				"L" => self.facing = Direction::South,
				_ => {}
			},
			Direction::South => match direction {
				"R" => self.facing = Direction::West,
				"L" => self.facing = Direction::East,
				_ => {}
			},
			Direction::East => match direction {
				"R" => self.facing = Direction::South,
				"L" => self.facing = Direction::North,
				_ => {}
			},
		}
	}

	fn step(&mut self) {
		match self.facing {
			Direction::North => self.y += 1,
			Direction::West => self.x += 1,
			Direction::South => self.y -= 1,
			Direction::East => self.x -= 1,
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
		let result = invoke(&"R8, R4, R4, R8".to_string());
		assert_eq!(result, "4");
	}
}
