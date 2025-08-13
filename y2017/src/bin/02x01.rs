use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2017/02.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let items = line.split_whitespace().collect::<Vec<_>>();
        let values = items
            .iter()
            .map(|v| v.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let min = values.iter().min().unwrap();
        let max = values.iter().max().unwrap();
        sum += max - min;
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "5 1 9 5
7 5 3
2 4 6 8";
        let result = invoke(input);
        assert_eq!(result, "18");
    }
}
