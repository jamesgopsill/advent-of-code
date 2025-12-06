fn main() {
    let input = include_str!("../../../puzzle_data/2025/06.txt").trim();
    let out = invoke(input);
    println!("{out}");
}

fn invoke(input: &str) -> String {
    let mut operation: Vec<char> = Vec::new();
    let mut calcs: Vec<u64> = Vec::new();
    let mut lines = input.lines().rev();

    // First line.
    let line = lines.next().expect("Expected a line");
    let cols = line.split_whitespace();
    for col in cols {
        operation.push(col.chars().next().expect("Should be a char"));
    }

    // get initial value
    let line = lines.next().expect("Expected a line");
    let cols = line.split_whitespace();
    for col in cols {
        calcs.push(col.parse().expect("Should be a number"));
    }

    for line in lines {
        let cols = line.split_whitespace();
        for (i, col) in cols.enumerate() {
            let val: u64 = col.parse().expect("Should parse as a number");
            match operation[i] {
                '*' => calcs[i] *= val,
                '+' => calcs[i] += val,
                _ => panic!("Should not get here."),
            }
        }
    }

    calcs.iter().sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";
        let result = invoke(input);
        assert_eq!(result, "4277556");
    }
}
