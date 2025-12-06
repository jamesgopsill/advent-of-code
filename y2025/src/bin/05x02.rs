fn main() {
    let input = include_str!("../../../puzzle_data/2025/05.txt").trim();
    let out = invoke(input);
    println!("{out}");
}

fn invoke(input: &str) -> String {
    let lines = input.lines();
    let mut ranges: Vec<Range> = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let (left, right) = line.split_once("-").unwrap();
        let left: u64 = left.parse().unwrap();
        let right: u64 = right.parse().unwrap();
        /*
        if left >= right {
            panic!("Left {left} >= Right {right}")
        }
        */
        ranges.push(Range::new(left, right));
    }

    // Iterate and merge the ranges.
    let mut count = ranges.len();
    loop {
        println!("{:?}", ranges);
        ranges = merge(ranges);
        // Check if the ranges could no longer be merged.
        if count == ranges.len() {
            break;
        } else {
            count = ranges.len();
        }
    }

    // count the ids
    let mut sum = 0;
    for r in ranges {
        sum += r.right - r.left + 1;
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

fn merge(ranges: Vec<Range>) -> Vec<Range> {
    let mut new_range: Vec<Range> = Vec::new();
    for pri_range in ranges {
        let mut merged = false;
        // Checks if a prior range can be merged with another.
        for cur_range in new_range.iter_mut() {
            // If contained within another.
            /*
            if pri_range.left >= cur_range.left && pri_range.right <= cur_range.right {
                consumed = true;
                break;
            }
            */
            // If the left is within a range and the right extends it.
            if pri_range.left >= cur_range.left
                && pri_range.left <= cur_range.right
                && pri_range.right >= cur_range.right
            {
                cur_range.right = pri_range.right;
                merged = true;
                break;
            }
            // If the right is within a range and the left extends its.
            if pri_range.right <= cur_range.right
                && pri_range.right >= cur_range.left
                && pri_range.left <= cur_range.left
            {
                cur_range.left = pri_range.left;
                merged = true;
                break;
            }
        }
        // Added if not consumed.
        if !merged {
            new_range.push(pri_range);
        }
    }
    new_range
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
