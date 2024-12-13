use regex::Regex;

pub fn invoke(input: &String) -> String {
	let re = Regex::new(r"[-\d]+").unwrap();
	let caps = re.captures_iter(input.as_str());
	let mut sum: i32 = 0;
	for cap in caps {
		let value = cap.get(0).unwrap().as_str().parse::<i32>().unwrap();
		sum += value;
	}
	sum.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(&"[1,2,3]".to_string());
		assert_eq!(result, "6");
	}

	#[test]
	fn test_b() {
		let result = invoke(&r#"{"a":2,"b":4}"#.to_string());
		assert_eq!(result, "6");
	}

	#[test]
	fn test_c() {
		let result = invoke(&r#"{"a":[-1,1]}"#.to_string());
		assert_eq!(result, "0");
	}
}
