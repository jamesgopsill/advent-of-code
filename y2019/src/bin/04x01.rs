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
        let mut chars = pass.chars();
        let mut increasing = true;
        let mut adjacent = false;
        let mut current = chars.next().unwrap().to_digit(10).unwrap();
        for c in chars {
            let next = c.to_digit(10).unwrap();
            if next < current {
                increasing = false;
                break;
            }
            if current == next {
                adjacent = true;
            }
            current = next;
        }
        if adjacent && increasing {
            count += 1;
        }
    }
    count.to_string()
}

#[cfg(test)]
mod tests {}
