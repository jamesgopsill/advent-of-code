use regex::Regex;
use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2015/06.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut lights = [[false; 1_000]; 1_000];
	let action_re = Regex::new(r"turn on|toggle|turn off").unwrap();
	let coords_re = Regex::new(r"\d+,\d+").unwrap();

	for line in input.lines() {
		let action = action_re.find(line).unwrap().as_str();
		let mut coords = coords_re.find_iter(line);
		let from = coords.next().unwrap().as_str();
		let from = parse_coords(from);
		let to = coords.next().unwrap().as_str();
		let to = parse_coords(to);
		for x in from[0]..=to[0] {
			for y in from[1]..=to[1] {
				match action {
					"turn on" => lights[x][y] = true,
					"turn off" => lights[x][y] = false,
					"toggle" => match lights[x][y] {
						true => lights[x][y] = false,
						false => lights[x][y] = true,
					},
					_ => {
						println!("Unknown action")
					}
				}
			}
		}
	}

	let mut count: u32 = 0;
	for row in lights.iter() {
		for bulb in row {
			if *bulb {
				count += 1
			}
		}
	}

	count.to_string()
}

fn parse_coords(s: &str) -> [usize; 2] {
	let values: Vec<&str> = s.split(",").collect();
	[
		values[0].parse::<usize>().unwrap(),
		values[1].parse::<usize>().unwrap(),
	]
}

#[cfg(test)]
mod tests_0601 {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("turn on 0,0 through 999,999");
		assert_eq!(result, "1000000");
	}

	#[test]
	fn test_b() {
		let result = invoke("toggle 0,0 through 999,0");
		assert_eq!(result, "1000");
	}

	#[test]
	fn test_c() {
		let result = invoke("turn off 499,499 through 500,500");
		assert_eq!(result, "0");
	}
}
