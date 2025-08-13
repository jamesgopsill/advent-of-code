use std::{collections::HashMap, sync::LazyLock};

use regex::Regex;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2015/07.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

type WireMap = HashMap<String, Wire>;

// TODO: cache the result for repeated looks down the wires.
// Puzzle seems to run for a long time.
// Caching now fails the unit test.
fn invoke(input: &str) -> String {
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
        if *key == "b" {
            wire.instruction = ans.to_string();
        }
    }
    compute_value("a".to_string(), &mut wires).to_string()
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

    fn update_cache(&mut self, value: Option<u16>) {
        self.cached_value = value;
    }
}

static ACTION_RE: LazyLock<Regex> = LazyLock::new(|| {
    println!("Initializing Action Re");
    Regex::new(r"(\w+)\s(AND|OR|LSHIFT|RSHIFT)\s(\w+)").unwrap()
});

fn compute_value(key: String, wires: &mut WireMap) -> u16 {
    let mut wire = wires.get(&key).cloned().unwrap();
    if wire.cached_value.is_some() {
        return wire.cached_value.unwrap();
    }

    let caps = ACTION_RE.captures(wire.instruction.as_str());
    if let Some(caps) = caps {
        let left = caps.get(1).unwrap().as_str().to_string();
        let left_num = left.parse::<u16>();
        let left_value: u16 = match left_num {
            Ok(left_num) => left_num,
            Err(_) => compute_value(left, wires),
        };

        let right = caps.get(3).unwrap().as_str().to_string();
        let right_num = right.parse::<u16>();
        let right_value: u16 = match right_num {
            Ok(right_num) => right_num,
            Err(_) => compute_value(right, wires),
        };

        let action = caps.get(2).unwrap().as_str();

        match action {
            "AND" => {
                let value = left_value & right_value;
                wire.update_cache(Some(value));
                wires.insert(key, wire);
                return value;
            }
            "OR" => {
                let value = left_value | right_value;
                wire.update_cache(Some(value));
                wires.insert(key, wire);
                return value;
            }
            "LSHIFT" => {
                let value = left_value << right_value;
                wire.update_cache(Some(value));
                wires.insert(key, wire);
                return value;
            }
            "RSHIFT" => {
                let value = left_value >> right_value;
                wire.update_cache(Some(value));
                wires.insert(key, wire);
                return value;
            }
            _ => {
                panic!("How did we get here?!")
            }
        }
    }

    // Deal with other pass-throughs, signals and NOTS
    let is_number = wire.instruction.parse::<u16>();
    if let Ok(number) = is_number {
        println!("Signal Reached");
        wire.update_cache(Some(number));
        wires.insert(key, wire);
        return number;
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
                wire.update_cache(Some(value));
                wires.insert(key, wire);
                return value;
            }
            Err(_) => {
                let value = !compute_value(right.to_string(), wires);
                wire.update_cache(Some(value));
                wires.insert(key, wire);
                return value;
            }
        }
    }

    // Pass through
    compute_value(wire.instruction, wires)
}
