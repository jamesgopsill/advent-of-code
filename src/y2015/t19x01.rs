use std::collections::HashMap;

use itertools::Itertools;

pub fn invoke(input: &String) -> String {
	let mut lines: Vec<&str> = input.lines().collect();
	let mut replacements: HashMap<&str, Vec<&str>> = HashMap::new();
	// Create the list of changes
	for line in lines.iter() {
		if line.len() == 0 {
			break;
		}
		let (a, b) = line.split_once("=>").unwrap();
		let a = a.trim();
		let b = b.trim();
		if replacements.contains_key(a) {
			let r = replacements.get_mut(a).unwrap();
			r.push(b);
		} else {
			replacements.insert(a, vec![b]);
		}
	}
	let mut molecule = lines.pop().unwrap().to_string();
	// provide some more values to extend into
	molecule.push_str("----------");
	let mut molecules = vec![];
	for (k, v) in replacements.iter() {
		let idxs: Vec<usize> = molecule.match_indices(k).map(|(i, _)| i).collect();
		//println!("{} {:?}", k, idxs);
		for idx in idxs {
			for r in v {
				println!("{} {}", k, r);
				let mut m = molecule.clone();
				m.replace_range(idx..idx + k.len(), r);
				molecules.push(m);
			}
		}
	}
	println!("{:?}", molecules.len());
	let molecules = molecules.iter().unique().collect::<Vec<&String>>();
	println!("{:?}", molecules.len());
	molecules.len().to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			&"H => HO
H => OH
O => HH

HOH
"
			.to_string(),
		);
		assert_eq!(result, "4");
	}
}
