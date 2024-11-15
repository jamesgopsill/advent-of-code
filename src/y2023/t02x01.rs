use regex::Regex;

pub fn invoke(input: String) -> u32 {
	let mut out: u32 = 0;
	let lines = input.lines();
	let gid_reg = Regex::new(r"Game\s(\d+)").unwrap();
	let balls_reg = Regex::new(r"(\d+)\s([brg])").unwrap();
	for line in lines {
		// println!("---");
		// dbg!(line);

		// Game Id
		let game = gid_reg.captures(line).unwrap();
		let gid = game[1].parse::<u32>().unwrap();
		// dbg!(gid);

		// Ball Selection
		let mut valid = true;
		let caps = balls_reg.captures_iter(line);
		for cap in caps {
			// dbg!(&cap);
			let number = cap[1].parse::<u32>().unwrap();
			let colour = &cap[2];
			match colour {
				"r" => {
					if number > 12 {
						valid = false
					}
				}
				"g" => {
					if number > 13 {
						valid = false
					}
				}
				"b" => {
					if number > 14 {
						valid = false;
					}
				}
				_ => {
					println!("Should not get here!")
				}
			}
			if !valid {
				break;
			};
		}
		// dbg!(gid, valid);
		if valid {
			out += gid;
		}
	}
	out
}

#[cfg(test)]
mod tests {
	use std::fs;

	use super::invoke;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/2023/02x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input);
		assert_eq!(result, 8);
	}
}
