use std::fs::read_to_string;
use std::time::Instant;

pub mod a01;
pub mod a02;
pub mod a03;
pub mod a04;
pub mod a05;
pub mod a06;
pub mod a07;
pub mod template;

pub fn run_advent(puzzle_path: &str, fcn: fn(s: String, d: bool) -> u32, debug: bool) {
    let input = read_to_string(puzzle_path).expect("Should have been able to read the file");
    let now = Instant::now();
    let result = fcn(input, debug);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Part 01 Result: {}", result);
}
