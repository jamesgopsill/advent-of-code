use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/13.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();

    // create the patterns
    let mut patterns: Vec<Pattern> = vec![];
    let mut tmp: Vec<String> = vec![];
    for line in lines {
        if line.is_empty() {
            tmp.push(line.to_string());
        } else {
            let pattern = Pattern::new(tmp.clone());
            patterns.push(pattern);
            tmp.clear();
        }
    }
    let pattern = Pattern::new(tmp.clone());
    patterns.push(pattern);
    tmp.clear();
    println!("{patterns:?}");
    for pattern in patterns {
        let mirror_index = pattern.row_reflection();
        match mirror_index {
            Some(index) => {
                println!("Mirrored about {index}");
            }
            None => {
                println!("No mirror");
            }
        }
    }

    0.to_string()
}

#[derive(Debug)]
struct Pattern {
    pattern: Vec<String>,
}

impl Pattern {
    fn new(pattern: Vec<String>) -> Self {
        Pattern { pattern }
    }

    fn row_reflection(&self) -> Option<u32> {
        for i in 1..self.pattern.len() {
            println!("Checking mirror @ {i}");
            let mut above = i - 1;
            let mut below = i;
            let mut mirrored = true;
            loop {
                let a = &self.pattern[above];
                let b = &self.pattern[below];
                if *a != *b {
                    mirrored = false;
                    break;
                }
                if above == 0 {
                    break;
                }
                if below == self.pattern.len() - 1 {
                    break;
                }
                above -= 1;
                below += 1;
            }
            if mirrored {
                return Some(i as u32);
            }
        }
        None
    }

    fn _col_reflection(&self) -> Option<u32> {
        None
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test() {
        let input = fs::read_to_string("test_data/2023/13x01.txt")
            .expect("Should have been able to read the file");
        let result = invoke(input, true);
        assert_eq!(result, 405);
    }
}
*/
