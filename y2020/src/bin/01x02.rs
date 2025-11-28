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
        if *a > 2020 {
            break;
        }
        for (j, b) in values[i + 1..].iter().enumerate() {
            if a + b > 2020 {
                break;
            }
            for c in values[j + 1..].iter() {
                if a + b + c == 2020 {
                    return (a * b * c).to_string();
                }
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
        assert_eq!(result, "241861950");
    }
}
