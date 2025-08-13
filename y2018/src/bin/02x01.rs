use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2018/02.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut twos: u32 = 0;
    let mut threes: u32 = 0;

    for line in input.lines() {
        let mut two = false;
        let mut three = false;
        for c in 'a'..='z' {
            let m = line.matches(c);
            let count = m.count();
            match count {
                2 => two = true,
                3 => three = true,
                _ => {}
            }
        }
        if two {
            twos += 1;
        }
        if three {
            threes += 1;
        }
    }

    let ans = twos * threes;

    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";
        let result = invoke(input);
        assert_eq!(result, "12");
    }
}
