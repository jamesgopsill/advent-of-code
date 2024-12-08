use std::{collections::HashMap, u32};

pub fn invoke(input: &String) -> String {
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
		let mut final_count = u32::MAX;
		for (c, count) in map {
			if final_count > count {
				final_c = c;
				final_count = count;
			}
		}
		error_corrected_message.push(final_c);
	}
	error_corrected_message.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			&"eedadn
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
"
			.to_string(),
		);
		assert_eq!(result, "advent");
	}
}
