use itertools::Itertools;

pub fn invoke(input: &String) -> String {
	let mut decompressed_string: Vec<char> = vec![];
	let mut buffer: Vec<char> = vec![];
	let mut iter = input.trim().chars();
	let mut copy: u32 = 0;
	let mut repeats: u32;

	while let Some(c) = iter.next() {
		match c {
			'(' => {
				decompressed_string.extend(&buffer);
				buffer.clear();
			}
			'x' => {
				let val = buffer.iter().join("");
				copy = val.parse().unwrap();
				buffer.clear();
			}
			')' => {
				let val = buffer.iter().join("");
				repeats = val.parse().unwrap();
				buffer.clear();
				// copy the next set of values and repeat them
				// into the decompressed string. Movec the iter
				// automatically to the next read point.
				for _ in 0..copy {
					buffer.push(iter.next().unwrap());
				}
				for _ in 0..repeats {
					decompressed_string.extend(&buffer);
				}
				buffer.clear();
			}
			_ => {
				buffer.push(c);
			}
		}
	}
	decompressed_string.extend(&buffer);
	buffer.clear();

	//println!("{}", decompressed_string.iter().join(""));
	decompressed_string.len().to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "ADVENT".to_string();
		let result = invoke(&input);
		assert_eq!(result, "6");
	}

	#[test]
	fn test_b() {
		let input = "A(1x5)BC".to_string();
		let result = invoke(&input);
		assert_eq!(result, "7");
	}

	#[test]
	fn test_c() {
		let input = "(3x3)XYZ".to_string();
		let result = invoke(&input);
		assert_eq!(result, "9");
	}

	#[test]
	fn test_d() {
		let input = "A(2x2)BCD(2x2)EFG".to_string();
		let result = invoke(&input);
		assert_eq!(result, "11");
	}

	#[test]
	fn test_e() {
		let input = "(6x1)(1x3)A".to_string();
		let result = invoke(&input);
		assert_eq!(result, "6");
	}

	#[test]
	fn test_f() {
		let input = "X(8x2)(3x3)ABCY".to_string();
		let result = invoke(&input);
		assert_eq!(result, "18");
	}
}
