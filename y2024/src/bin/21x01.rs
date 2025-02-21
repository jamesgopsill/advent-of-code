use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use utils::bench;

// TODO: expand the options for the commands
// as there may be more optimum sequences of commands
// itertools permutations.

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/21.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let re = Regex::new(r"\d+").unwrap();
	let mut codes: Vec<(u32, Vec<char>)> = vec![];
	for line in input.lines() {
		let number = re.captures(line).unwrap();
		let number = number.get(0).unwrap().as_str().parse::<u32>().unwrap();

		let mut code: Vec<char> = vec![];
		for c in line.chars() {
			code.push(c);
		}
		codes.push((number, code));
	}

	let ans: u32 = 0;
	for (_num, code) in codes {
		println!("{:?}", code);

		// Value

		// Keypad Robot
		println!("Dpad Robot to Keypad Robot");
		let mut cmds: Vec<Vec<char>> = vec![vec![]];
		let mut current = 'A';
		for next in code {
			let mut nxt_cmds: Vec<Vec<char>> = vec![];
			let options = generate_cmd_options(current, next);
			for cmd in cmds.iter() {
				for option in options.iter() {
					let mut nxt = cmd.clone();
					nxt.extend(option);
					nxt_cmds.push(nxt);
				}
			}
			cmds = nxt_cmds;
			current = next;
		}

		//let cmds = keypad_cmds(code);
		println!("Options: {}", cmds.len());
		println!("{:?}", cmds);

		// Robot one
		/*
		println!("Dpad to Dpad Robot");
		let cmds = dpad_cmds(cmds);
		//println!("{:?}", cmds);
		println!("{}", cmds.len());

		println!("Historian Dpad to Dpad Robot");
		let cmds = dpad_cmds(cmds);
		println!("{:?}", cmds);
		println!("{}", cmds.len());

		println!(
			"Complexity: {} {} {}",
			cmds.len(),
			num,
			cmds.len() as u32 * num
		);

		ans += cmds.len() as u32 * num;
		*/

		// ??? Thought I would need to do it again
		//println!("Dpad Historian");
		//let cmds = dpad_cmds(cmds);
		//println!("{:?}", cmds);
		//println!("{}", cmds.len());
	}

	ans.to_string()
}

fn generate_cmd_options(
	current: char,
	next: char,
) -> Vec<Vec<char>> {
	// Combined keypad.
	// Move to a lazy init.
	let mut keypad: HashMap<char, (i32, i32)> = HashMap::new();
	keypad.insert('7', (0, 0));
	keypad.insert('8', (0, 1));
	keypad.insert('9', (0, 2));
	keypad.insert('4', (1, 0));
	keypad.insert('5', (1, 1));
	keypad.insert('6', (1, 2));
	keypad.insert('1', (2, 0));
	keypad.insert('2', (2, 1));
	keypad.insert('3', (2, 2));
	keypad.insert('0', (3, 1));
	keypad.insert('A', (3, 2)); // Used by both
	keypad.insert('^', (3, 1));
	keypad.insert('>', (3, 3));
	keypad.insert('<', (0, 3));
	keypad.insert('v', (1, 3));
	// no-go
	let nogo = (3, 0);

	let current = keypad.get(&current).unwrap();
	let next = keypad.get(&next).unwrap();

	let mut cmd_set = vec![];
	let row_move = current.0 - next.0;
	if row_move > 0 {
		for _ in 0..row_move {
			cmd_set.push('^');
		}
	}
	if row_move < 0 {
		for _ in 0..row_move.abs() {
			cmd_set.push('v');
		}
	}
	let col_move = current.1 - next.1;
	if col_move > 0 {
		for _ in 0..col_move {
			cmd_set.push('<');
		}
	}
	if col_move < 0 {
		for _ in 0..col_move.abs() {
			cmd_set.push('>');
		}
	}

	let combinations = cmd_set.iter().permutations(cmd_set.len());
	let mut valid_patterns: Vec<Vec<char>> = vec![];
	for combination in combinations {
		let mut p = current.clone();
		let mut valid = true;
		for c in combination.iter() {
			match c {
				'^' => p.0 -= 1,
				'>' => p.1 += 1,
				'v' => p.0 += 1,
				'<' => p.1 -= 1,
				_ => {
					panic!("Should not get here")
				}
			}
			if p == nogo {
				println!("Invalid Path");
				valid = false;
				break;
			}
		}
		if valid {
			let mut pattern: Vec<char> = vec![];
			pattern.extend(combination);
			pattern.push('A');
			valid_patterns.push(pattern);
		}
	}
	valid_patterns
}

/*
fn generate_cmds(cmds: Vec<char>) -> Vec<Vec<char>> {
	// Combined keypad.
	let mut keypad: HashMap<char, (i32, i32)> = HashMap::new();
	keypad.insert('7', (0, 0));
	keypad.insert('8', (0, 1));
	keypad.insert('9', (0, 2));
	keypad.insert('4', (1, 0));
	keypad.insert('5', (1, 1));
	keypad.insert('6', (1, 2));
	keypad.insert('1', (2, 0));
	keypad.insert('2', (2, 1));
	keypad.insert('3', (2, 2));
	keypad.insert('0', (3, 1));
	keypad.insert('A', (3, 2)); // Used by both
	keypad.insert('^', (3, 1));
	keypad.insert('>', (3, 3));
	keypad.insert('<', (0, 3));
	keypad.insert('v', (1, 3));
	// no-go
	let nogo = (3, 0);

	let mut next_cmds: Vec<Vec<char>> = vec![];

	let mut position: char = 'A';
	for c in cmds.iter() {
		let current = keypad.get(&position).unwrap();
		let next = keypad.get(&c).unwrap();

		// Create the cmds options.
		let mut cmd_set = vec![];
		let row_move = current.0 - next.0;
		if row_move > 0 {
			for _ in 0..row_move {
				cmd_set.push('^');
			}
		}
		if row_move < 0 {
			for _ in 0..row_move.abs() {
				cmd_set.push('v');
			}
		}
		let col_move = current.1 - next.1;
		if col_move > 0 {
			for _ in 0..col_move {
				cmd_set.push('<');
			}
		}
		if col_move < 0 {
			for _ in 0..col_move.abs() {
				cmd_set.push('>');
			}
		}

		// Create the combinations
		let combinations = cmd_set.iter().permutations(cmd_set.len());
		for combination in combinations {
			let mut p = keypad.get(&position).unwrap().clone();
			let mut valid = true;
			for c in combination.iter() {
				match c {
					'^' => p.0 -= 1,
					'>' => p.1 += 1,
					'v' => p.0 += 1,
					'<' => p.1 -= 1,
					_ => {
						panic!("Should not get here")
					}
				}
				if p == nogo {
					valid = false;
					break;
				}
			}
			if valid {
				let mut next_cmd = cmds.clone();
				next_cmd.extend(combination);
				next_cmd.push('A');
				next_cmds.push(next_cmd)
			}
		}
	}
	next_cmds
}

fn keypad_cmds(cmds: Vec<char>) -> Vec<char> {
	let mut position: char = 'A';
	let mut keypad_cmds: Vec<char> = vec![];
	let mut keypad: HashMap<char, (i32, i32)> = HashMap::new();
	keypad.insert('7', (0, 0));
	keypad.insert('8', (0, 1));
	keypad.insert('9', (0, 2));
	keypad.insert('4', (1, 0));
	keypad.insert('5', (1, 1));
	keypad.insert('6', (1, 2));
	keypad.insert('1', (2, 0));
	keypad.insert('2', (2, 1));
	keypad.insert('3', (2, 2));
	keypad.insert('0', (3, 1));
	keypad.insert('A', (3, 2));
	for c in cmds {
		let current = keypad.get(&position).unwrap();
		let next = keypad.get(&c).unwrap();
		// println!("{:?} {:?}", current, next);
		//
		//
		/*
		let row_move = current.0 - next.0;
		if row_move > 0 {
			for _ in 0..row_move {
				keypad_cmds.push('^');
			}
		}
		if row_move < 0 {
			for _ in 0..row_move.abs() {
				keypad_cmds.push('v');
			}
		}
		let col_move = current.1 - next.1;
		if col_move > 0 {
			for _ in 0..col_move {
				keypad_cmds.push('<');
			}
		}
		if col_move < 0 {
			for _ in 0..col_move.abs() {
				keypad_cmds.push('>');
			}
		}
		*/
		keypad_cmds.push('A');
		position = c;
	}
	keypad_cmds
}

fn dpad_cmds(cmds: Vec<char>) -> Vec<char> {
	let mut position: char = 'A';
	let mut dpad_cmds: Vec<char> = vec![];

	let mut dpad: HashMap<char, (i32, i32)> = HashMap::new();
	dpad.insert('^', (0, 1));
	dpad.insert('>', (1, 2));
	dpad.insert('<', (1, 0));
	dpad.insert('v', (1, 1));
	dpad.insert('A', (0, 2));

	for cmd in cmds {
		let current = dpad.get(&position).unwrap();
		let next = dpad.get(&cmd).unwrap();
		// println!("{:?} {:?}", current, next);
		let row_move = current.0 - next.0;
		let col_move = current.1 - next.1;
		if col_move > 0 {
			for _ in 0..col_move {
				dpad_cmds.push('<');
			}
		}
		if col_move < 0 {
			for _ in 0..col_move.abs() {
				dpad_cmds.push('>');
			}
		}
		if row_move > 0 {
			for _ in 0..row_move {
				dpad_cmds.push('^');
			}
		}
		if row_move < 0 {
			for _ in 0..row_move.abs() {
				dpad_cmds.push('v');
			}
		}
		dpad_cmds.push('A');
		position = cmd;
	}
	dpad_cmds
}
*/

#[cfg(test)]
mod tests_21x01 {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "029A".to_string();
		let result = invoke(&input);
		assert_eq!(result, "1972");
	}

	/*
		#[test]
		fn test_b() {
			let input = "029A
	980A
	179A
	456A
	379A"
				.to_string();
			let result = invoke(&input);
			assert_eq!(result, "126384");
		}
		*/
}
