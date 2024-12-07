use md5::{Digest, Md5};

pub fn invoke(input: &String) -> String {
	let mut password: String = String::new();
	let mut n: u32 = 0;
	let input = input.trim();
	println!("{}", input);
	let mut hasher = Md5::new();
	for i in 0..9_999_999 {
		let secret = format!("{}{}", input, i);
		hasher.update(secret.as_bytes());
		let result = hasher.finalize_reset();
		let hash = format!("{:x}", result);
		if hash.starts_with("00000") {
			println!("Found One");
			let letter = &hash[5..6];
			password.push_str(letter);
			n += 1;
			if n == 8 {
				break;
			}
		}
	}
	password
}

/*
#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "abc".to_string();
		let result = invoke(&input);
		assert_eq!(result, "18f47a30");
	}
}
*/
