use advent::a03::part_01;

use std::fs::read_to_string;

fn main() {
    let input =
        read_to_string("data/puzzle_03.txt").expect("Should have been able to read the file");
    let result = part_01(input);
    println!("Part 01 Result: {}", result);
}
