use regex::Regex;

struct Game {
	r: u32,
	g: u32,
	b: u32,
}

pub fn invoke(
	input: String,
	debug: bool,
) -> u32 {
	let mut out: u32 = 0;

	let lines = input.lines();
	let balls_reg = Regex::new(r"(\d+)\s([brg])").unwrap();

	for line in lines {
		if debug {
			dbg!(line);
		}
		let mut game = Game { r: 0, g: 0, b: 0 };
		let caps = balls_reg.captures_iter(line);
		for cap in caps {
			let number = cap[1].parse::<u32>().unwrap();
			let colour = &cap[2];
			match colour {
				"r" => {
					if number > game.r {
						game.r = number;
					}
				}
				"g" => {
					if number > game.g {
						game.g = number;
					}
				}
				"b" => {
					if number > game.b {
						game.b = number;
					}
				}
				_ => {
					println!("Should not get here!")
				}
			}
		}
		out += game.r * game.g * game.b;
	}
	out
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/2023/02x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input, true);
		assert_eq!(result, 2286);
	}
}
