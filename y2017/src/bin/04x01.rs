use std::{collections::HashSet, fs};
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2017/04.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let passphrases = input.lines();
    let mut count = 0;
    for phrase in passphrases {
        if is_valid(phrase) {
            count += 1;
        }
    }
    count.to_string()
}

fn is_valid(s: &str) -> bool {
    let words: Vec<&str> = s.split_whitespace().collect();
    let set: HashSet<&str> = HashSet::from_iter(words.iter().cloned());
    words.len() == set.len()
}

#[cfg(test)]
mod tests {
    use crate::is_valid;

    #[test]
    fn test_a() {
        let input = "aa bb cc dd ee";
        let valid = is_valid(input);
        assert!(valid);
    }

    #[test]
    fn test_b() {
        let input = "aa bb cc dd aa";
        let valid = is_valid(input);
        assert!(!valid);
    }

    #[test]
    fn test_c() {
        let input = "aa bb cc dd aaa";
        let valid = is_valid(input);
        assert!(valid);
    }
}
