use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2025/01.txt").trim();
    let out = invoke(input);
    println!("{out}");
    bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let mut password = 0;
    let mut position: i32 = 50;
    let lines = input.trim().lines();
    for line in lines {
        let rotate: i32 = line[1..].parse().unwrap();
        match line.chars().next().unwrap() {
            'L' => {
                for _ in 0..rotate {
                    position -= 1;
                    if position == 0 {
                        password += 1;
                    }
                    if position == -1 {
                        position = 99;
                    }
                }
            }
            'R' => {
                for _ in 0..rotate {
                    position += 1;
                    if position == 100 {
                        position = 0;
                        password += 1;
                    }
                }
            }
            _ => panic!("Should not get here."),
        }
    }
    password.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let result = invoke(input);
        assert_eq!(result, "6");
    }
}
