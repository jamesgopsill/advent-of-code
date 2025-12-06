fn main() {
    let input = include_str!("../../../puzzle_data/2025/05.txt").trim();
    let out = invoke(input);
    println!("{out}");
}

fn invoke(input: &str) -> String {
    // Gather the ranges
    let lines = input.lines();
    let mut ranges: Vec<Range> = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let (left, right) = line.split_once("-").unwrap();
        let left: u64 = left.parse().unwrap();
        let right: u64 = right.parse().unwrap();
        if left >= right {
            println!("Left {left} >= Right {right}")
        }
        ranges.push(Range::new(left, right));
    }

    // Sort the ranges
    ranges.sort_by_key(|r| r.right);
    ranges.sort_by_key(|r| r.left);

    // Iterate through either extending or moving
    // to the next range.
    let mut sum = 0;
    let mut id = 0;
    for r in ranges {
        if r.right <= id {
            // already passed it
            continue;
        }
        // If we're jumping a gap
        if r.left > id {
            sum += r.right - r.left + 1;
            id = r.right;
        } else {
            // we're extending the existing range.
            sum += r.right - id;
            id = r.right;
        }
    }

    sum.to_string()
}

#[derive(Debug)]
struct Range {
    left: u64,
    right: u64,
}

impl Range {
    fn new(left: u64, right: u64) -> Self {
        Self { left, right }
    }
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
        assert_eq!(result, "14");
    }
}
