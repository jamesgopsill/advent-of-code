use std::fs;
use utils::bench_val;

fn main() {
    let input = fs::read_to_string("puzzle_data/2015/10.txt").unwrap();
    let out = invoke(&input, 50);
    println!("{}", out);
    bench_val(invoke, &input, 50);
}

fn invoke(
    input: &str,
    times: u64,
) -> String {
    // Rust regex does not support backreferencing -> (\w)\1+
    // https://stackoverflow.com/questions/12258622/regular-expression-to-check-for-repeating-characters#12258829
    // Could use another package but I think we could group this ourselves.

    let mut sequence = input.trim().to_string();
    println!("{}", sequence);
    for _ in 0..times {
        // Identify the groups
        let mut groups: Vec<(char, u32)> = vec![];
        let mut current_char: char = 'Z';
        let mut count: u32 = 0;
        for char in sequence.chars() {
            if current_char == char {
                count += 1;
            } else {
                if current_char != 'Z' {
                    let group = (current_char, count);
                    groups.push(group);
                }
                current_char = char;
                count = 1;
            }
        }
        // append the last group
        let group = (current_char, count);
        groups.push(group);

        // Create the new sequence
        let mut new_sequence = String::new();
        for (char, count) in groups {
            let count = format!("{}", count);
            new_sequence.push_str(count.as_str());
            new_sequence.push(char);
        }
        sequence = new_sequence
    }
    // println!("{}", sequence);
    sequence.len().to_string()
}
