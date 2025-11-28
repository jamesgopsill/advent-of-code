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
        let pwd = elements[2];
        let c = elements[1].chars().next().unwrap();
        let (left, right) = elements[0].split_once("-").unwrap();
        let left: usize = left.parse().unwrap();
        let right: usize = right.parse().unwrap();
        let left = pwd.chars().nth(left - 1).unwrap();
        let right = pwd.chars().nth(right - 1).unwrap();
        if (left == c || right == c) && left != right {
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
        assert_eq!(result, "1");
    }
}
