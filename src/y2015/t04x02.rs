use md5::{Digest, Md5};

pub fn invoke(input: &String) -> String {
	let input = input.trim();
	let mut hasher = Md5::new();
	for i in 0..9_999_999 {
		let secret = format!("{}{}", input, i);
		hasher.update(secret.as_bytes());
		let result = hasher.finalize_reset();
		let hash = format!("{:x}", result);
		if hash.starts_with("000000") {
			println!("{} {}", secret, hash);
			return i.to_string();
		}
	}
	0.to_string()
}
