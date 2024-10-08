use regex::Regex;

#[derive(Clone, Debug)]
struct MapRange {
	from_lower: u64,
	from_upper: u64,
	to_lower: u64,
}

pub fn invoke(input: String) -> u64 {
	let lines: Vec<&str> = input.lines().collect();
	let numbers_re = Regex::new(r"\d+").unwrap();
	let seeds: Vec<u64> = numbers_re
		.find_iter(lines[0])
		.map(|f| f.as_str().parse::<u64>().unwrap())
		.collect();
	println!("Seeds: {:?}", seeds);

	let mut maps: Vec<Vec<MapRange>> = vec![];
	let mut map: Vec<MapRange> = vec![];
	for line in lines[2..].into_iter() {
		if line.contains("map") {
			map.clear();
			continue;
		}
		if line.len() == 0 {
			maps.push(map.clone());
			continue;
		}
		let range: Vec<u64> = numbers_re
			.find_iter(line)
			.map(|f| f.as_str().parse::<u64>().unwrap())
			.collect();
		println!("{:?}", line);
		println!("{:?}", range);
		let map_range = MapRange {
			from_lower: range[1],
			to_lower: range[0],
			from_upper: range[1] + range[2] - 1,
		};
		map.push(map_range);
		println!("{}", line)
	}
	maps.push(map.clone());

	//println!("{:?}", maps);
	let mut loc: u64 = 999_999_999;
	for seed in seeds {
		let mut location = seed.clone();
		for map in &maps {
			for rng in map {
				if rng.from_lower <= location && location <= rng.from_upper {
					location = rng.to_lower + (location - rng.from_lower);
					break;
				}
			}
		}
		println!("From: {}, To: {}", seed, location);
		if location < loc {
			loc = location
		}
	}

	loc
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/05x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input);
		assert_eq!(result, 35);
	}
}
