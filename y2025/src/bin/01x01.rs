use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2025/01.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "";
        let result = invoke(input);
        assert_eq!(result, "");
    }
}
