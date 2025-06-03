use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2018/01.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut value = 0;
    // Could split.map(trim.parse).collect but
    // can process them one at a time.
    for frequency_change in input.split_whitespace() {
        let frequency_change = frequency_change.trim().parse::<i32>().unwrap();
        value += frequency_change;
    }
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "+1\n+1\n+1";
        let result = invoke(input);
        assert_eq!(result, "3");
    }

    #[test]
    fn test_b() {
        let input = "+1\n+1\n-2";
        let result = invoke(input);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_c() {
        let input = "-1\n-2\n-3";
        let result = invoke(input);
        assert_eq!(result, "-6");
    }
}
