use regex::Regex;

pub fn convert_to_u32(value: &str) -> u32 {
    match value {
        "nine" => return 9,
        "eight" => return 8,
        "seven" => return 7,
        "six" => return 6,
        "five" => return 5,
        "four" => return 4,
        "three" => return 3,
        "two" => return 2,
        "one" => return 1,
        _ => return value.parse::<u32>().unwrap(),
    }
}

pub fn part_02(input: String, debug: bool) -> u32 {
    let mut sum: u32 = 0;
    let lines = input.lines();
    let fn_re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let ln_re = Regex::new(r"^.*([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    for line in lines {
        let first = fn_re.find(&line).unwrap().as_str();
        let last = &ln_re.captures(&line).unwrap()[1];
        if debug {
            dbg!(first, last);
        }

        let first = convert_to_u32(first);
        let last = convert_to_u32(last);

        let number = format!("{}{}", first, last).parse::<u32>().unwrap();
        if debug {
            dbg!(line, number);
        }
        sum += number;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_02() {
        let input = fs::read_to_string("src/a01/input_02.txt")
            .expect("Should have been able to read the file");
        let result = part_02(input, true);
        assert_eq!(result, 281);
    }
}
