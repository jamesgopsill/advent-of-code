# Advent of Code in Rust

My attempts at solving the [Advent of Code](https://adventofcode.com) challenges in Rust.

## Running

To run the examples against your puzzle data. Clone the repo and make a `puzzle_data` folder in the repo root. Add your puzzle data here and then call the right function against the puzzle data.

```
cargo run -p y2024 --bin 01x01
```

```
cargo test -p y2024 --bin 01x01
```

## Template

All the functions start their life from this template.

```rust
fn main() {
	let input = fs::read_to_string("puzzle_data/[XXXX]/[YY].txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(
	input: &str
) -> u32 {
	todo!()
}

#[cfg(test)]
mod tests {
    use super::invoke

	#[test]
	fn test() {
		let input: str = "test_input";
		let result = invoke(input);
		assert_eq!(result, 142);
	}
}
```
