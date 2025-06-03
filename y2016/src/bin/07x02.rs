use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2016/07.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    // Create all the aba, aca patterns re
    // due to no overlapping check limit of regex.
    let mut aba_patterns: Vec<Regex> = vec![];
    let mut bab_patterns: Vec<Regex> = vec![];
    for p in ('a'..='z').permutations(2) {
        let mut pattern = "".to_string();
        pattern.push(p[0]);
        pattern.push(p[1]);
        pattern.push(p[0]);
        aba_patterns.push(Regex::new(pattern.as_str()).unwrap());
        let mut pattern = "".to_string();
        pattern.push(p[1]);
        pattern.push(p[0]);
        pattern.push(p[1]);
        bab_patterns.push(Regex::new(pattern.as_str()).unwrap());
    }
    let brackets_re = Regex::new(r"\[\w+\]").unwrap();
    let mut ans: u32 = 0;
    for line in input.lines() {
        //println!("{}", line);
        // Check what exists in each bracketed section
        let mut in_bracket: HashSet<usize> = HashSet::new();
        for cap in brackets_re.captures_iter(line) {
            let s = cap.get(0).unwrap().as_str();
            for (i, p) in aba_patterns.iter().enumerate() {
                if p.is_match(s) {
                    in_bracket.insert(i);
                }
            }
        }
        //println!("{:?}", in_bracket);
        // Remove the brackets from the line
        let line = brackets_re.replace_all(line, "-").to_string();
        // Check if the pattern in the bracket
        // exists outside the brackets
        for i in in_bracket {
            if bab_patterns[i].is_match(&line) {
                ans += 1;
                break;
            }
        }
    }
    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb";
        let result = invoke(input);
        assert_eq!(result, "3");
    }
}
