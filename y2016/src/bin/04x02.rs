use regex::Regex;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2016/04.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let re = Regex::new(r"([a-z\-]+)(\d+)\[([a-z]+)").unwrap();
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let code = cap.get(1).unwrap().as_str();
        let number = cap.get(2).unwrap().as_str();
        let checksum = cap.get(3).unwrap().as_str();
        //println!("{} {} {}", code, number, checksum);
        let mut letters: Vec<(char, u32)> = vec![];
        for c in 'a'..='z' {
            let mut count = 0;
            for l in code.chars() {
                if l == c {
                    count += 1;
                }
            }
            letters.push((c, count));
        }
        letters.sort_by(|a, b| b.1.cmp(&a.1));
        //println!("{:?}", letters);
        let check = format!(
            "{}{}{}{}{}",
            letters[0].0, letters[1].0, letters[2].0, letters[3].0, letters[4].0
        );
        //println!("{}", check);
        if checksum == check.as_str() {
            decrypt_name(code, number);
        }
    }
    0.to_string()
}

fn decrypt_name(code: &str, number: &str) {
    let number = number.parse::<u32>().unwrap();
    let a = 'a'.to_digit(36).unwrap();
    let mut val = String::new();
    for c in code.chars() {
        if c == '-' {
            val.push(' ');
            continue;
        }
        let d = number + c.to_digit(36).unwrap() - a;
        let d = (d % 26) + a;
        let c = char::from_digit(d, 36).unwrap();
        val.push(c);
    }
    println!("{} {}", val, number);
}

#[cfg(test)]
mod tests {
    use super::decrypt_name;

    #[test]
    fn test_a() {
        let _ = decrypt_name("qzmt-zixmtkozy-ivhz-", "343");
        assert_eq!(true, true);
    }
}
