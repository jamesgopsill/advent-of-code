pub fn invoke(input: String) -> String {
	let lines = input.lines();
	let mut p = PinPad::new();
	let mut passcode: String = String::new();
	for line in lines {
		for c in line.chars() {
			p.step(c);
		}
		match p.pin {
			Pin::One => passcode.push_str("1"),
			Pin::Two => passcode.push_str("2"),
			Pin::Three => passcode.push_str("3"),
			Pin::Four => passcode.push_str("4"),
			Pin::Five => passcode.push_str("5"),
			Pin::Six => passcode.push_str("6"),
			Pin::Seven => passcode.push_str("7"),
			Pin::Eight => passcode.push_str("8"),
			Pin::Nine => passcode.push_str("9"),
			Pin::A => passcode.push_str("A"),
			Pin::B => passcode.push_str("B"),
			Pin::C => passcode.push_str("C"),
			Pin::D => passcode.push_str("D"),
		}
	}
	passcode
}

enum Pin {
	One,
	Two,
	Three,
	Four,
	Five,
	Six,
	Seven,
	Eight,
	Nine,
	A,
	B,
	C,
	D,
}

struct PinPad {
	pin: Pin,
}

impl PinPad {
	fn new() -> Self {
		Self { pin: Pin::Five }
	}

	fn step(
		&mut self,
		c: char,
	) {
		match self.pin {
			Pin::One => match c {
				'D' => self.pin = Pin::Three,
				_ => {}
			},
			Pin::Two => match c {
				'R' => self.pin = Pin::Three,
				'D' => self.pin = Pin::Six,
				_ => {}
			},
			Pin::Three => match c {
				'R' => self.pin = Pin::Four,
				'L' => self.pin = Pin::Two,
				'D' => self.pin = Pin::Seven,
				'U' => self.pin = Pin::One,
				_ => {}
			},
			Pin::Four => match c {
				'L' => self.pin = Pin::Three,
				'D' => self.pin = Pin::Eight,
				_ => {}
			},
			Pin::Five => match c {
				'R' => self.pin = Pin::Six,
				_ => {}
			},
			Pin::Six => match c {
				'R' => self.pin = Pin::Seven,
				'L' => self.pin = Pin::Five,
				'D' => self.pin = Pin::A,
				'U' => self.pin = Pin::Two,
				_ => {}
			},
			Pin::Seven => match c {
				'R' => self.pin = Pin::Eight,
				'L' => self.pin = Pin::Six,
				'D' => self.pin = Pin::B,
				'U' => self.pin = Pin::Three,
				_ => {}
			},
			Pin::Eight => match c {
				'R' => self.pin = Pin::Nine,
				'L' => self.pin = Pin::Seven,
				'D' => self.pin = Pin::C,
				'U' => self.pin = Pin::Four,
				_ => {}
			},
			Pin::Nine => match c {
				'L' => self.pin = Pin::Eight,
				_ => {}
			},
			Pin::A => match c {
				'R' => self.pin = Pin::B,
				'U' => self.pin = Pin::Six,
				_ => {}
			},
			Pin::B => match c {
				'R' => self.pin = Pin::C,
				'L' => self.pin = Pin::A,
				'D' => self.pin = Pin::D,
				'U' => self.pin = Pin::Seven,
				_ => {}
			},
			Pin::C => match c {
				'L' => self.pin = Pin::B,
				'U' => self.pin = Pin::Eight,
				_ => {}
			},
			Pin::D => match c {
				'U' => self.pin = Pin::B,
				_ => {}
			},
		};
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			"ULL
RRDDD
LURDL
UUUUD
"
			.to_string(),
		);
		assert_eq!(result, "5DB3");
	}
}
