use std::fs;
//use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2018/05.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    //bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut min_length: usize = usize::MAX;
    for c in 'a'..='z' {
        let mut polymer = input.trim().to_string();
        polymer = polymer.replace(c, "");
        polymer = polymer.replace(c.to_ascii_uppercase(), "");
        let l = react(polymer);
        if l < min_length {
            min_length = l;
        }
    }

    min_length.to_string()
}

fn react(mut polymer: String) -> usize {
    let mut doubles = Vec::new();
    for c in 'a'..='z' {
        doubles.push(format!("{}{}", c, c.to_uppercase()));
        doubles.push(format!("{}{}", c.to_uppercase(), c));
    }
    loop {
        let before = polymer.len();
        for dbl in doubles.iter() {
            polymer = polymer.replace(dbl, "");
        }
        let after = polymer.len();
        if before == after {
            break;
        }
    }
    polymer.len()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "dabAcCaCBAcCcaDA";
        let result = invoke(input);
        assert_eq!(result, "4");
    }
}
