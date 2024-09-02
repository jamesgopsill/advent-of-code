pub fn part_01(input: String) -> i32 {
    let mut sum: i32 = 0;
    let lines = input.lines();
    for line in lines {
        let digits: Vec<&str> = line.matches(char::is_numeric).collect();
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let number = format!("{}{}", first, last).parse::<i32>().unwrap();
        dbg!("{} {}", line, number);
        sum += number;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_01() {
        let input = fs::read_to_string("src/a01/input_01.txt")
            .expect("Should have been able to read the file");
        let result = part_01(input);
        assert_eq!(result, 142);
    }
}
