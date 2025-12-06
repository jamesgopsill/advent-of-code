fn main() {
    let input = include_str!("../../../puzzle_data/2025/05.txt").trim();
    let out = invoke(input);
    println!("{out}");
}

fn invoke(input: &str) -> String {
    let mut lines = input.lines();
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (left, right) = line.split_once("-").unwrap();
        let left: u64 = left.parse().unwrap();
        let right: u64 = right.parse().unwrap();
        ranges.push((left, right));
    }

    // Now check the values
    let mut fresh = 0;
    while let Some(line) = lines.next() {
        let id: u64 = line.parse().unwrap();
        for (left, right) in ranges.iter() {
            if id >= *left && id <= *right {
                fresh += 1;
                break;
            }
        }
    }

    fresh.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let result = invoke(input);
        assert_eq!(result, "3");
    }
}
