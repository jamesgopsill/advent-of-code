use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2025/01.txt").trim();
    let out = invoke(input);
    println!("{out}");
    bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let mut pwd: i32 = 0;
    let mut pos: i32 = 50;
    let lines = input.trim().lines();
    for line in lines {
        let rot: i32 = line[1..].parse().unwrap();
        let quo = rot.div_euclid(100).abs();
        pwd += quo;
        let rem = rot % 100;
        match line.chars().next().unwrap() {
            'L' => pos += rem,
            'R' => pos -= rem,
            _ => panic!("Should not get here"),
        }
        if pos == 0 {
            pwd += 1
        } else {
            let quo = pos.div_euclid(100).abs();
            pwd += quo;
            pos %= 100;
        }
    }
    pwd.to_string()
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
