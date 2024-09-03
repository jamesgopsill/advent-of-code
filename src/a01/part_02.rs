use regex::Regex;

pub fn part_02(input: String) -> u32 {
    let mut sum: u32 = 0;
    let lines = input.lines();
    let numbers_re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    for line in lines {
        let numbers: Vec<u32> = numbers_re
            .find_iter(&line)
            .map(|f| {
                let v = f.as_str();
                // dbg!(v);
                match v {
                    "nine" => return 9,
                    "eight" => return 8,
                    "seven" => return 7,
                    "six" => return 6,
                    "five" => return 5,
                    "four" => return 4,
                    "three" => return 3,
                    "two" => return 2,
                    "one" => return 1,
                    _ => return v.parse::<u32>().unwrap(),
                }
            })
            .collect();
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        let number = format!("{}{}", first, last).parse::<u32>().unwrap();
        dbg!(line, number);
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
        let result = part_02(input);
        assert_eq!(result, 281);
    }
}
