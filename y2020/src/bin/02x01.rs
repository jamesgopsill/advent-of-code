use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2020/02.txt");
    let out = invoke(input);
    println!("{out}");
    bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let lines = input.trim().lines();
    let mut valid: u32 = 0;
    for line in lines {
        let elements: Vec<&str> = line.split_whitespace().collect();
        let (left, right) = elements[0].split_once("-").unwrap();
        let min: u32 = left.parse().unwrap();
        let max: u32 = right.parse().unwrap();
        let c = elements[1].chars().next().unwrap();
        let pwd = elements[2];
        let count = pwd.chars().filter(|p| *p == c).count() as u32;
        if count >= min && count <= max {
            valid += 1;
        }
    }
    valid.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let result = invoke(input);
        assert_eq!(result, "2");
    }
}
