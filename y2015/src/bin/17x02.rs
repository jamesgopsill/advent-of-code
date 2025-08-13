use itertools::Itertools;
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle_data/2015/17.txt").unwrap();
    let out = invoke(&input, 150);
    println!("{}", out);
}

fn invoke(input: &str, liters: u32) -> String {
    let mut minimum_number: usize = 9_999;
    let mut variations: u32 = 0;
    let mut containers: Vec<u32> = vec![];
    for line in input.lines() {
        containers.push(line.parse::<u32>().unwrap());
    }
    for c in containers.iter().powerset() {
        let n = c.len();
        let mut sum: u32 = 0;
        for v in c {
            sum += *v;
        }
        if sum == liters {
            if n < minimum_number {
                minimum_number = n;
                variations = 1;
            } else if n == minimum_number {
                variations += 1;
            }
        }
    }
    variations.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "20
15
10
5
5";
        let result = invoke(input, 25);
        assert_eq!(result, "3");
    }
}
