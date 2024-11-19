# Advent of Code in Rust

My attempts at solving the [Advent of Code](https://adventofcode.com) challenges in Rust.

## Running

To run the examples against your puzzle data. Clone the repo and make a `puzzle_data` folder in the repo root. Add your puzzle data here and then call the right function against the puzzle data.

```
cargo run -- -y 2015 -d 1 -p 1 -f puzzle_data/2015/01.txt
```

- `-y` Year
- `-d` Day
- `-p` Part
- `-f` Puzzle Data FilePath

Use cargo test to check all the functions against the examples provided in the Advent of Code documentation.

```
cargo test
```

## Template

All the functions start their life from this template.

```rust
pub fn invoke(
	input: String
) -> u32 {
	todo!()
}

#[cfg(test)]
mod tests {
    use super::invoke

	#[test]
	fn test() {
		let input = "test_input".to_string();
		let result = invoke(input);
		assert_eq!(result, 142);
	}
}
```
