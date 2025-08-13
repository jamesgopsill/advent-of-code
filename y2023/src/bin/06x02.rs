use regex::Regex;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/06.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
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

    println!("{wins}");
    wins.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = invoke(input);
        assert_eq!(result, "288");
    }
}
