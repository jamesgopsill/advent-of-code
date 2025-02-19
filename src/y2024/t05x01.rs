use std::collections::HashMap;

pub fn invoke(input: &String) -> String {
	let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
	let mut pages: Vec<Vec<u32>> = vec![];

	for line in input.lines() {
		if line.contains("|") {
			let (a, b) = line.split_once("|").unwrap();
			let a = a.parse::<u32>().unwrap();
			let b = b.parse::<u32>().unwrap();
			if let Some(v) = rules.get_mut(&a) {
				v.push(b)
			} else {
				rules.insert(a, vec![b]);
			}
			continue;
		}
		if line.len() > 0 {
			let mut page: Vec<u32> = vec![];
			for item in line.split(",") {
				let item = item.parse::<u32>().unwrap();
				page.push(item);
			}
			pages.push(page);
		}
	}

	// Now to check each line
	let mut sum: u32 = 0;
	for page in pages {
		let mut valid = true;
		for (i, v) in page.iter().enumerate() {
			if let Some(rule_set) = rules.get(v) {
				for r in rule_set {
					for j in 0..i {
						if *r == page[j] {
							valid = false;
						}
					}
				}
			}
		}
		if valid {
			// get the middle number.
			let middle = page.len() / 2;
			sum += page[middle];
		}
	}
	sum.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"
		.to_string();
		let result = invoke(&input);
		assert_eq!(result, "143");
	}
}
