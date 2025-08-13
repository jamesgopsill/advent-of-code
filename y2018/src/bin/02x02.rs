use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2018/02.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    for a in input.lines() {
        for b in input.lines() {
            let (d, c) = diff(a, b);
            // println!("{} {} {} {}", a, b, d, c);
            if d == 1 {
                return c.to_string();
            }
        }
    }
    "".to_string()
}

fn diff(a: &str, b: &str) -> (u32, String) {
    let mut common = String::new();
    let mut diff: u32 = 0;
    let a_chars = a.chars();
    let b_chars = b.chars();
    for (a, b) in a_chars.zip(b_chars) {
        if a != b { diff += 1 } else { common.push(a) }
    }
    (diff, common)
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";
        let result = invoke(input);
        assert_eq!(result, "fgij");
    }
}
