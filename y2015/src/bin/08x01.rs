use regex::Regex;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2015/08.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut literal_chars: u32 = 0;
    let mut memory_chars: u32 = 0;
    let backslash = Regex::new(r"\\\\").unwrap();
    let hexdecimal = Regex::new(r"\\x[\w]{2}").unwrap();
    let quote = Regex::new(r#"\\""#).unwrap();
    for line in input.lines() {
        let literal = backslash.replace_all(line, "X");
        let literal = format!("{}", literal);
        let literal = literal.as_str();
        let literal = hexdecimal.replace_all(literal, "Y");
        let literal = format!("{}", literal);
        let literal = literal.as_str();
        let literal = quote.replace_all(literal, "Z");
        let literal = format!("{}", literal);
        let literal = literal.as_str();
        println!("{} {}", line, literal);

        let mem_chars = line.len() as u32;
        let lit_chars = literal.len() as u32 - 2;
        println!("{} {} {} {}", line, mem_chars, literal, lit_chars);

        memory_chars += mem_chars;
        literal_chars += lit_chars;
    }
    (memory_chars - literal_chars).to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let result = invoke(
            "\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"
",
        );
        assert_eq!(result, "12");
    }
}
