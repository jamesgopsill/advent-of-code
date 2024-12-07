use md5::{Digest, Md5};

pub fn invoke(input: String) -> u32 {
	let mut hasher = Md5::new();
	for i in 0..9_999_999 {
		let secret = format!("{}{}", input.trim(), i);
		hasher.update(secret.as_bytes());
		let result = hasher.finalize_reset();
		let hash = format!("{:x}", result);
		if hash.starts_with("00000") {
			println!("{} {}", secret, hash);
			return i;
		}
	}
	0
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke("abcdef".to_string());
		assert_eq!(result, 609043);
	}

	#[test]
	fn test_b() {
		let result = invoke("pqrstuv".to_string());
		assert_eq!(result, 1048970);
	}
}
