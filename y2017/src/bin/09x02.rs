use std::fs;
//use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2017/09.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    //bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut garbage: bool = false;
    let mut ignore: bool = false;
    let mut garbage_count: u64 = 0;
    for char in input.chars() {
        if ignore {
            ignore = false;
            continue;
        }

        match char {
            '!' => ignore = true,
            '<' => {
                if !garbage {
                    garbage = true;
                    continue;
                }
                garbage_count += 1;
            }
            '>' => {
                if garbage {
                    garbage = false;
                    continue;
                }
                garbage_count += 1;
            }
            _ => {
                if garbage {
                    garbage_count += 1;
                }
            }
        }
    }

    garbage_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = r"<random characters>";
        let result = invoke(input);
        assert_eq!(result, "17");
    }

    #[test]
    fn test_b() {
        let input = "<{o\"i!a,<{i<a>";
        let result = invoke(input);
        assert_eq!(result, "10");
    }
}
