use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2019/01.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let lines = input.lines();
    let mut sum: u64 = 0;
    for line in lines {
        let mass: f64 = line.parse().unwrap();
        let mut fuel = (mass / 3.).floor() as i64 - 2;
        while fuel > 0 {
            sum += fuel as u64;
            fuel = (fuel as f64 / 3.).floor() as i64 - 2;
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "12";
        let result = invoke(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_b() {
        let input = "1969";
        let result = invoke(input);
        assert_eq!(result, "966");
    }

    #[test]
    fn test_c() {
        let input = "100756";
        let result = invoke(input);
        assert_eq!(result, "50346");
    }
}
