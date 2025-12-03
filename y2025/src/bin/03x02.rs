fn main() {
    let input = include_str!("../../../puzzle_data/2025/03.txt").trim();
    let out = invoke(input);
    println!("{out}");
}

fn invoke(input: &str) -> String {
    let mut sum: u64 = 0;
    let lines = input.lines();
    for line in lines {
        let mut indices: Vec<usize> = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let c_len = chars.len();

        let mut cur_idx = 0;

        for digit in (0..12).rev() {
            let mut max = 0;
            let mut digit_idx = 0;
            let end = c_len - digit;
            for (j, c) in chars[cur_idx..end].iter().enumerate() {
                let val: u32 = c.to_digit(10).unwrap();
                if val > max {
                    max = val;
                    digit_idx = cur_idx + j;
                }
            }
            indices.push(digit_idx);
            cur_idx = digit_idx + 1;
        }

        let mut joltage = String::new();
        for i in indices {
            joltage.push(chars[i])
        }
        let joltage: u64 = joltage.parse().unwrap();
        sum += joltage;
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
        assert_eq!(result, "3121910778619");
    }
}
