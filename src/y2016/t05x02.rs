use md5::{Digest, Md5};

pub fn invoke(input: &String) -> String {
	let mut password: [char; 8] = ['-'; 8];
	let input = input.trim();
	println!("{}", input);
	let mut hasher = Md5::new();
	for i in 0..99_999_999 {
		let secret = format!("{}{}", input, i);
		hasher.update(secret.as_bytes());
		let result = hasher.finalize_reset();
		let hash = format!("{:x}", result);
		if hash.starts_with("00000") {
			println!("Found One");
			println!("{}", hash);
			let loc = hash.chars().nth(5).unwrap();
			let c = hash.chars().nth(6).unwrap();
			println!("{} {}", loc, c);
			let loc = loc.to_digit(10);
			if loc.is_some() {
				let loc = loc.unwrap() as usize;
				if loc > 7 {
					continue;
				}
				if password[loc] == '-' {
					password[loc] = c;
					if !password.contains(&'-') {
						println!("Password Broken");
						break;
					}
				}
			}
		}
	}
	password.iter().collect::<String>()
}

/*
#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "abc".to_string();
		let result = invoke(&input);
		assert_eq!(result, "05ace8e3");
	}
}
*/
