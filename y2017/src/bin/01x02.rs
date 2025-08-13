use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2017/01.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut values: Vec<u32> = vec![];
    for c in input.trim().chars() {
        let v = c.to_digit(10).unwrap();
        values.push(v);
    }
    let count = values.len();
    let middle = count / 2;
    values.extend_from_within(0..);
    let mut sum: u32 = 0;
    for i in 0..count {
        if values[i] == values[i + middle] {
            sum += values[i]
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "1212";
        let result = invoke(input);
        assert_eq!(result, "6");
    }

    #[test]
    fn test_b() {
        let input = "1221";
        let result = invoke(input);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_c() {
        let input = "123425";
        let result = invoke(input);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_d() {
        let input = "123123";
        let result = invoke(input);
        assert_eq!(result, "12");
    }

    #[test]
    fn test_e() {
        let input = "12131415";
        let result = invoke(input);
        assert_eq!(result, "4");
    }
}
