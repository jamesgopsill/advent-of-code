use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2018/05.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut polymer = input.trim().to_string();

    let mut doubles = Vec::new();
    for c in 'a'..='z' {
        doubles.push(format!("{}{}", c, c.to_uppercase()));
        doubles.push(format!("{}{}", c.to_uppercase(), c));
    }

    let mut n: u32 = 0;

    while n < 1_000_000 {
        n += 1;
        let before = polymer.len();
        for dbl in doubles.iter() {
            polymer = polymer.replace(dbl, "");
        }
        let after = polymer.len();
        if before == after {
            break;
        }
    }

    //println!("{}", n);

    polymer.len().to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "dabAcCaCBAcCcaDA";
        let result = invoke(input);
        assert_eq!(result, "10");
    }
}
