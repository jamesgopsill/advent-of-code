use std::collections::HashMap;

// Time to cache in
pub fn invoke(input: &String) -> String {
	let mut lines = input.lines();
	// Get the stripes
	let mut stripes = lines
		.next()
		.unwrap()
		.split(",")
		.collect::<Vec<&str>>()
		.iter()
		.map(|f| f.trim())
		.collect::<Vec<&str>>();
	stripes.sort_by(|a, b| b.len().cmp(&a.len()));
	println!("{:?}", stripes);

	// Filter the stripes for combinations of other stripes
	let mut filtered_stripes: Vec<&str> = vec![];
	for (i, s) in stripes.iter().enumerate() {
		if s.len() == 1 {
			filtered_stripes.push(s);
			continue;
		}
		let valid = depth_first_search(s, &stripes[(i + 1)..], 0);
		println!("{}", valid);
		if !valid {
			filtered_stripes.push(s);
		}
	}
	println!("{:?}", filtered_stripes);

	// Check which towels we can make.
	lines.next(); // skip the empty line in the input
	let mut possible: Vec<&str> = vec![];
	for t in lines {
		println!("Towel: {}", t);
		let valid = depth_first_search(t, &filtered_stripes, 0);
		if valid {
			// println!("Possible");
			possible.push(t);
		} else {
			// println!("Impossible");
		}
	}

	// now some up all the combinations for that towel
	// Implement a cache to prevent repeat competitions
	// for strings we have already tried.
	let mut count: u64 = 0;
	let mut cache: HashMap<&str, u64> = HashMap::new();
	for t in possible {
		println!("Towel: {}", t);
		let c = dfs_count(t, &stripes, &mut cache);
		println!("{}", c);
		println!("Cache: {}", cache.len());
		count += c;
	}
	count.to_string()
}

fn depth_first_search(
	towel: &str,
	stripes: &[&str],
	i: usize,
	//d: usize,
) -> bool {
	if i == towel.len() {
		return true;
	}
	for s in stripes.iter() {
		if towel[i..].starts_with(s) {
			let next = depth_first_search(towel, stripes, i + s.len());
			if next {
				return true;
			}
		}
	}
	false
}

fn dfs_count<'a>(
	towel: &'a str,
	stripes: &[&str],
	cache: &mut HashMap<&'a str, u64>,
) -> u64 {
	if towel.len() == 0 {
		return 1;
	}
	let cache_value = cache.get(&towel);
	match cache_value {
		Some(val) => return *val,
		None => {
			let mut count = 0;
			for s in stripes {
				if towel.starts_with(s) {
					count += dfs_count(&towel[s.len()..], stripes, cache);
				}
			}
			cache.insert(towel, count);
			return count;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
			.to_string();
		let result = invoke(&input);
		assert_eq!(result, "16");
	}
}
