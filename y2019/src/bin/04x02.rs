#![allow(unused)]
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2019/04.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    //bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let (min, max) = input.trim().split_once(",").unwrap();
    let min: u32 = min.parse().unwrap();
    let max: u32 = max.parse().unwrap();
    let mut count = 0;
    for pass in min..=max {
        let pass = pass.to_string();
        let mut chars: Vec<u32> = pass.chars().map(|f| f.to_digit(10).unwrap()).collect();
        let mut increasing = true;
        for win in chars.windows(2) {
            if win[1] < win[0] {
                increasing = false;
                break;
            }
        }
        if !increasing {
            continue;
        }
        let mut adjacent_group = false;
        if chars[0] == chars[1] && chars[1] != chars[2] {
            count += 1;
            continue;
        }
        if chars[5] == chars[4] && chars[4] != chars[3] {
            count += 1;
            continue;
        }
        for win in chars.windows(4) {
            if win[0] != win[1] && win[1] == win[2] && win[2] != win[3] {
                count += 1;
                break;
            }
        }
    }
    count.to_string()
}

#[cfg(test)]
mod tests {}
