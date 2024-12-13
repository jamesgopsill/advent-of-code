use regex::Regex;

pub fn invoke(input: &String) -> String {
	let mut encoded_total: u32 = 0;
	let mut original_total: u32 = 0;
	let backslash = Regex::new(r"\\").unwrap();
	//let hexdecimal = Regex::new(r"\\x[\w]{2}").unwrap();
	let quote = Regex::new(r#"""#).unwrap();
	for line in input.lines() {
		let literal = backslash.replace_all(line, "\\\\");
		let literal = format!("{}", literal);
		let literal = literal.as_str();

		let rep = r#"\""#;
		let literal = quote.replace_all(literal, rep);
		let literal = format!("\"{}\"", literal);
		let literal = literal.as_str();

		let original_chars = line.len() as u32;
		let encoded_chars = literal.len() as u32;
		println!("{} {} {} {}", line, original_chars, literal, encoded_chars);
		encoded_total += encoded_chars;
		original_total += original_chars;
	}
	(encoded_total - original_total).to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let result = invoke(
			&"\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"
"
			.to_string(),
		);
		assert_eq!(result, "19");
	}
}
