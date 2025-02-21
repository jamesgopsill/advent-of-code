use std::collections::HashMap;
use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2016/06.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut messages: Vec<Vec<char>> = vec![];
	for line in input.lines() {
		messages.push(line.chars().collect());
	}
	let mut error_corrected_message: Vec<char> = vec![];
	for i in 0..messages[0].len() {
		let mut map: HashMap<char, u32> = HashMap::new();
		for message in messages.iter() {
			let c = message[i];
			let m = map.get_mut(&c);
			match m {
				Some(m) => *m += 1,
				_ => {
					let _ = map.insert(c, 0);
				}
			}
		}
		// find the largest
		let mut final_c = '#';
		let mut final_count = 0;
		for (c, count) in map {
			if final_count < count {
				final_c = c;
				final_count = count;
			}
		}
		error_corrected_message.push(final_c);
	}
	error_corrected_message.iter().collect::<String>()
}

#[cfg(test)]
mod tests_0601 {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			"eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar
",
		);
		assert_eq!(result, "easter");
	}
}
