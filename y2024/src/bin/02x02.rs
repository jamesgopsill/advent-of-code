use itertools::Itertools;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2024/02.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut total: u32 = 0;
    for report in input.lines() {
        let mut levels: Vec<i32> = vec![];
        let split = report.split_whitespace();
        for s in split {
            levels.push(s.parse::<i32>().unwrap())
        }
        if is_safe(&levels) {
            // println!("Safe without dampener: {:?}", levels);
            total += 1;
            continue;
        } else {
            let n = levels.len() - 1;
            let it = levels.into_iter().combinations(n);
            let mut safe: bool = false;
            for c in it {
                if is_safe(&c) {
                    safe = true;
                    break;
                }
            }
            // println!("{} {}", report, safe);
            if safe {
                total += 1;
            }
        }
    }
    total.to_string()
}

fn is_safe(levels: &[i32]) -> bool {
    let mut safe: bool = true;
    let mut count: i32 = 0;
    for win in levels.windows(2) {
        let diff = win[0].abs_diff(win[1]);
        if diff == 0 || diff > 3 {
            safe = false;
            break;
        }
        if win[0] - win[1] > 0 {
            count += 1;
        }
        if win[0] - win[1] < 0 {
            count -= 1;
        }
    }
    count = count.abs();
    if safe && count == (levels.len() - 1) as i32 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let result = invoke(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
",
        );
        assert_eq!(result, "4");
    }
}
