use std::fs;
//use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2017/09.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    //bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut clean_stream: String = String::new();
    let mut garbage: bool = false;
    let mut ignore: bool = false;
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
                }
            }
            '>' => {
                if garbage {
                    garbage = false;
                }
            }
            _ => {
                if !garbage {
                    clean_stream.push(char);
                }
            }
        }
    }

    //println!("{}", input);
    //println!("{}", clean_stream);

    // Count groups.
    let mut sum: u64 = 0;
    let mut level: u64 = 0;
    let mut ignore: bool = false;
    for char in clean_stream.chars() {
        if ignore {
            ignore = false;
            continue;
        }
        match char {
            '!' => ignore = true,
            '{' => level += 1,
            '}' => {
                sum += level;
                level -= 1;
            }
            _ => {}
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "{<a>,<a>,<a>,<a>}";
        let result = invoke(input);
        assert_eq!(result, "1");
    }

    #[test]
    fn test_b() {
        let input = "{{{},{},{{}}}}";
        let result = invoke(input);
        assert_eq!(result, "16");
    }
}
