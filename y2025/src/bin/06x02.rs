fn main() {
    // Note. make sure there is no last return value;
    let input = include_str!("../../../puzzle_data/2025/06.txt");
    let out = invoke(input);
    println!("{out}"); // 9434900032651
}

#[derive(Debug)]
pub enum OpCode {
    Multiply,
    Add,
}

#[derive(Debug)]
pub struct Op {
    op: OpCode,
    start: usize,
    end: usize,
}

impl Op {
    fn new(op: OpCode, start: usize, end: usize) -> Self {
        Self { op, start, end }
    }
}

fn invoke(input: &str) -> String {
    let mut lines: Vec<String> = input.lines().map(String::from).collect();
    let ops_line = lines.pop().expect("There should be a line");

    // Get the variable with operations
    let mut ops: Vec<Op> = Vec::new();
    let mut code = OpCode::Add;
    let mut start = 0;
    for (i, c) in ops_line.chars().enumerate() {
        match c {
            '*' => {
                if i != 0 {
                    let op = Op::new(code, start, i);
                    ops.push(op);
                }
                start = i;
                code = OpCode::Multiply;
            }
            '+' => {
                if i != 0 {
                    let op = Op::new(code, start, i);
                    ops.push(op);
                }
                start = i;
                code = OpCode::Add;
            }
            _ => {}
        }
    }
    // Add the last one
    let op = Op::new(code, start, ops_line.len() + 1);
    ops.push(op);

    let mut sum: u64 = 0;

    // Edge case fix. Pushing an extra space at the end of the line to make it like the other slices.
    for line in lines.iter_mut() {
        line.push(' ');
    }

    // Now, run through each operation and each line to gather and create the values.
    for op in ops {
        println!("{:?}", op);
        // Create the number of column strings based on the variable width
        // associated with the op.
        let mut values: Vec<String> = Vec::new();
        for _ in op.start..(op.end - 1) {
            values.push(String::new());
        }
        // Run down each line and each column appending
        // numeric values.
        for line in lines.iter() {
            let slice = &line[op.start..op.end];
            println!("{slice}");
            for (i, c) in slice.chars().enumerate() {
                if c.is_numeric() {
                    values[i].push(c);
                }
            }
        }

        // Convert the values into u64s.
        println!("{:?}", values);
        let values: Vec<u64> = values
            .iter()
            .map(|v| v.parse().expect("To be a number"))
            .collect();
        println!("{:?}", values);

        // Perform the operation.
        let result = match op.op {
            OpCode::Add => values.iter().sum(),
            OpCode::Multiply => {
                let mut result = 1;
                for v in values {
                    result *= v;
                }
                result
            }
        };

        // Add the to the sum.
        sum += result;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let result = invoke(input);
        assert_eq!(result, "3263827");
    }

    #[test]
    fn test_b() {
        let input = include_str!("../../../puzzle_data/2025/06.txt");
        let result = invoke(input);
        assert_eq!(result, "9434900032651");
    }
}
