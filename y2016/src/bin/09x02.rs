use itertools::Itertools;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2016/09.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let iter: Vec<char> = input.trim().chars().collect();
    let len = decompressed_length(iter);
    len.to_string()
}

fn decompressed_length(incoming: Vec<char>) -> u64 {
    println!("{}", incoming.iter().join(""));
    let mut len: u64 = 0;
    let mut copy: u64 = 0;
    let mut repeats: u32;
    let mut buffer: Vec<char> = vec![];
    let mut iter = incoming.iter();
    while let Some(c) = iter.next() {
        match c {
            '(' => {
                buffer.clear();
            }
            'x' => {
                let val = buffer.iter().join("");
                copy = val.parse().unwrap();
                buffer.clear();
            }
            ')' => {
                let val = buffer.iter().join("");
                repeats = val.parse().unwrap();
                buffer.clear();
                // copy the next set of values and repeat them
                // into the decompressed string. Movec the iter
                // automatically to the next read point.
                for _ in 0..copy {
                    buffer.push(*iter.next().unwrap());
                }
                let l = decompressed_length(buffer.clone());
                println!("return: {}", l);
                for _ in 0..repeats {
                    len += l;
                }
                buffer.clear();
            }
            '0'..='9' => {
                buffer.push(*c);
            }
            _ => {
                buffer.push(*c);
                len += 1;
                println!("{} {}", c, len);
            }
        }
    }
    len
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "(3x3)XYZ";
        let result = invoke(input);
        assert_eq!(result, "9");
    }

    #[test]
    fn test_b() {
        let input = "X(8x2)(3x3)ABCY";
        let result = invoke(input);
        assert_eq!(result, "20");
    }

    #[test]
    fn test_c() {
        let input = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
        let result = invoke(input);
        assert_eq!(result, "241920");
    }

    #[test]
    fn test_d() {
        let input = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
        let result = invoke(input);
        assert_eq!(result, "445");
    }
}
