use itertools::Itertools;
use std::{collections::HashMap, fs};
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2017/07.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

pub struct Program<'a> {
	name: &'a str,
	parent: &'a str,
	children: Vec<&'a str>,
}

impl<'a> Program<'a> {
	pub fn new(name: &'a str) -> Self {
		Self {
			name,
			parent: "",
			children: vec![],
		}
	}
}

fn invoke(input: &str) -> String {
	let mut tower: HashMap<&str, Program> = HashMap::new();

	// Initialise the nodes
	for line in input.lines() {
		let p = line
			.split("(")
			.collect_vec()
			.first()
			.unwrap()
			.to_owned()
			.trim();
		let p = Program::new(p);
		tower.insert(p.name, p);
	}

	// Now for the edges
	for line in input.lines() {
		if !line.contains("->") {
			continue;
		}
		let parent = line
			.split("(")
			.collect_vec()
			.first()
			.unwrap()
			.to_owned()
			.trim();
		let children = line.split("->").collect_vec().last().unwrap().to_owned();
		let children = children.split(",").collect_vec();
		for child in children {
			let child = child.trim();
			let node = tower.get_mut(parent).unwrap();
			node.children.push(child);
			let node = tower.get_mut(child).unwrap();
			node.parent = parent;
		}
	}

	// Now find the root node
	for (_, v) in tower {
		if v.parent.is_empty() {
			return v.name.to_string();
		}
	}

	"".to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
		let result = invoke(input);
		assert_eq!(result, "tknk");
	}
}
