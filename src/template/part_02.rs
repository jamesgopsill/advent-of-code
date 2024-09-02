pub fn part_02(_: String) -> i32 {
    let sum: i32 = 0;
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_02() {
        let input = fs::read_to_string("src/a01/input_01.txt")
            .expect("Should have been able to read the file");
        let result = part_02(input);
        assert_eq!(result, 0);
    }
}
