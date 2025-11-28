use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2020/01.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let values: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    for (i, a) in values.iter().enumerate() {
        for b in values[i + 1..].iter() {
            if a + b == 2020 {
                return (a * b).to_string();
            }
        }
    }
    "".into()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "1721
979
366
299
675
1456";
        let result = invoke(input);
        assert_eq!(result, "514579");
    }
}
