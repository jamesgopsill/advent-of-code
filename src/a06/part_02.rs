use regex::Regex;

pub fn part_02(input: String) -> u64 {
    let numbers_re = Regex::new(r"[\d\s]+").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let time = numbers_re
        .find(lines[0])
        .unwrap()
        .as_str()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let distance = numbers_re
        .find(lines[1])
        .unwrap()
        .as_str()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    dbg!(&time);
    dbg!(&distance);

    let mut wins: u64 = 0;
    let mut dist: u64;

    for j in 1..time {
        dist = j * (time - j);
        if dist > distance {
            wins += 1;
        }
    }

    println!("{}", wins);
    wins
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_01() {
        let input = fs::read_to_string("src/a06/input_01.txt")
            .expect("Should have been able to read the file");
        let result = part_02(input);
        assert_eq!(result, 71_503);
    }
}
