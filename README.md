# Advent of Code 2023 in Rust

My attempts at solving the [Advent of Code 2023](https://adventofcode.com/2023) challenges in Rust.

## To run

```
cargo run -- -t 01x01 -f test_data/01x01.txt
```


## Template

```rust
pub fn invoke(
	input: String,
	debug: bool,
) -> u32 {
	todo!()
}

#[cfg(test)]
mod tests {
    use super::*
	use std::fs;

	#[test]
	fn test() {
		let input = fs::read_to_string("test_data/01x01.txt")
			.expect("Should have been able to read the file");
		let result = invoke(input, true);
		assert_eq!(result, 142);
	}
}
```