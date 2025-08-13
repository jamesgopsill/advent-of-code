use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2017/05.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn to_i32(v: &str) -> i32 {
    v.parse::<i32>().unwrap()
}

fn invoke(input: &str) -> String {
    let mut jumps: Vec<i32> = input.lines().map(to_i32).collect();
    let n = (jumps.len() - 1) as i32;
    let mut idx: usize = 0;
    let mut steps = 0;

    loop {
        steps += 1;
        let jump = jumps[idx];
        let next_idx = (idx as i32) + jump;
        if next_idx < 0 || next_idx > n {
            break;
        }
        jumps[idx] += 1;
        idx = next_idx as usize;
    }

    steps.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "0
3
0
1
-3";
        let result = invoke(input);
        assert_eq!(result, "5");
    }
}
