fn main() {
    let input = include_str!("../../../puzzle_data/2025/03.txt").trim();
    let out = invoke(input);
    println!("{out}");
}

fn invoke(input: &str) -> String {
    let mut sum: u32 = 0;
    let lines = input.lines();
    for line in lines {
        let mut joltage: u8 = 0;
        for (i, a) in line.chars().enumerate() {
            for b in line[i + 1..].chars() {
                let val: u8 = format!("{a}{b}").parse().unwrap();
                if val > joltage {
                    joltage = val;
                }
            }
        }
        sum += joltage as u32;
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let result = invoke(input);
        assert_eq!(result, "357");
    }
}
