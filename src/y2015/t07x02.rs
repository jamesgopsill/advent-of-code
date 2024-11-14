use std::collections::HashMap;

use regex::Regex;

type WireMap = HashMap<String, Wire>;

// TODO: cache the result for repeated looks down the wires.
// Puzzle seems to run for a long time.
// Caching now fails the unit test.
pub fn invoke(
	input: String,
	_debug: bool,
) -> u16 {
	let mut wires: WireMap = HashMap::new();
	for line in input.lines() {
		let (left, right) = line.split_once("->").unwrap();
		let right = right.trim();
		let left = left.trim();
		wires.insert(right.to_string(), Wire::new(left.to_string()));
	}
	let ans = compute_value("a".to_string(), &mut wires);
	for (key, wire) in &mut wires {
		wire.update_cache(None);
		if *key == "b".to_string() {
			wire.instruction = ans.to_string();
		}
	}
	compute_value("a".to_string(), &mut wires)
}

#[derive(Debug, Clone)]
struct Wire {
	instruction: String,
	cached_value: Option<u16>,
}

impl Wire {
	fn new(instruction: String) -> Self {
		Self {
			instruction,
			cached_value: None,
		}
	}

	fn update_cache(
		&mut self,
		value: Option<u16>,
	) {
		self.cached_value = value;
	}
}

fn compute_value(
	key: String,
	wires: &mut WireMap,
) -> u16 {
	let mut wire = wires.get(&key).cloned().unwrap();
	if wire.cached_value.is_some() {
		return wire.cached_value.unwrap().clone();
	}
	let re = Regex::new(r"(\w+)\s(AND|OR|LSHIFT|RSHIFT)\s(\w+)").unwrap();

	let caps = re.captures(wire.instruction.as_str());
	match caps {
		Some(caps) => {
			let left_value: u16;
			let left = caps.get(1).unwrap().as_str().to_string();
			let left_num = left.parse::<u16>();
			match left_num {
				Ok(left_num) => left_value = left_num,
				Err(_) => left_value = compute_value(left, wires),
			}

			let right_value: u16;
			let right = caps.get(3).unwrap().as_str().to_string();
			let right_num = right.parse::<u16>();
			match right_num {
				Ok(right_num) => right_value = right_num,
				Err(_) => right_value = compute_value(right, wires),
			}

			let action = caps.get(2).unwrap().as_str();

			match action {
				"AND" => {
					let value = left_value & right_value;
					wire.update_cache(Some(value.clone()));
					wires.insert(key, wire);
					return value;
				}
				"OR" => {
					let value = left_value | right_value;
					wire.update_cache(Some(value.clone()));
					wires.insert(key, wire);
					return value;
				}
				"LSHIFT" => {
					let value = left_value << right_value;
					wire.update_cache(Some(value.clone()));
					wires.insert(key, wire);
					return value;
				}
				"RSHIFT" => {
					let value = left_value >> right_value;
					wire.update_cache(Some(value.clone()));
					wires.insert(key, wire);
					return value;
				}
				_ => {
					panic!("How did we get here?!")
				}
			}
		}
		_ => {}
	}
	// Deal with other pass-throughs, signals and NOTS
	let is_number = wire.instruction.parse::<u16>();
	match is_number {
		Ok(number) => {
			println!("Signal Reached");
			wire.update_cache(Some(number.clone()));
			wires.insert(key, wire);
			return number;
		}
		_ => {}
	}

	// NOT instruction
	if wire.instruction.starts_with("NOT") {
		let right = wire.instruction.strip_prefix("NOT ").unwrap();
		let right = right.trim();
		let value: u16;
		let right_num = right.parse::<u16>();
		match right_num {
			Ok(right_num) => {
				value = !right_num;
				wire.update_cache(Some(value.clone()));
				wires.insert(key, wire);
				return value;
			}
			Err(_) => {
				let value = !compute_value(right.to_string(), wires);
				wire.update_cache(Some(value.clone()));
				wires.insert(key, wire);
				return value;
			}
		}
	}

	// Pass through
	return compute_value(wire.instruction, wires);
}
