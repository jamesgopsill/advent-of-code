use advent::a02::part_01;
use advent::a02::part_02;

use std::fs::read_to_string;

fn main() {
    let input =
        read_to_string("data/puzzle_02.txt").expect("Should have been able to read the file");
    let result = part_01(input);
    println!("Part 01 Result: {}", result);

    let input =
        read_to_string("data/puzzle_02.txt").expect("Should have been able to read the file");
    let result = part_02(input);
    println!("Part 02 Result: {}", result);
}
