use regex::Regex;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/01.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut sum: u32 = 0;
    let lines = input.lines();
    let fn_re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let ln_re = Regex::new(r"^.*([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    for line in lines {
        let first = fn_re.find(line).unwrap().as_str();
        let last = &ln_re.captures(line).unwrap()[1];
        // dbg!(first, last);

        let first = convert_to_u32(first);
        let last = convert_to_u32(last);

        let number = format!("{first}{last}").parse::<u32>().unwrap();
        // dbg!(line, number);
        sum += number;
    }
    sum.to_string()
}

fn convert_to_u32(value: &str) -> u32 {
    match value {
        "nine" => 9,
        "eight" => 8,
        "seven" => 7,
        "six" => 6,
        "five" => 5,
        "four" => 4,
        "three" => 3,
        "two" => 2,
        "one" => 1,
        _ => value.parse::<u32>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();
        let result = invoke(&input);
        assert_eq!(result, "281");
    }
}
