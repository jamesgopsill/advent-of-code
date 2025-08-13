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
    let mut sum: u32 = 1;
    let numbers_re = Regex::new(r"\d+").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<i32> = numbers_re
        .find_iter(lines[0])
        .map(|f| f.as_str().parse::<i32>().unwrap())
        .collect();
    let distances: Vec<i32> = numbers_re
        .find_iter(lines[1])
        .map(|f| f.as_str().parse::<i32>().unwrap())
        .collect();
    let n_races = times.len();
    let mut wins: u32;
    let mut dist: i32;
    for i in 0..n_races {
        let t = times[i];
        let d = distances[i];
        wins = 0;
        for j in 1..t {
            dist = j * (t - j);
            if dist > d {
                wins += 1;
            }
        }
        println!("{i} {wins}");
        sum *= wins;
    }
    sum.to_string()
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
