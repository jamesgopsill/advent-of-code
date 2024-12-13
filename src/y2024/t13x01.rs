use regex::Regex;

// Classic
// Part 1: Brute Force It. Part 2. I know what's coming.
pub fn invoke(input: &String) -> String {
	let re = Regex::new(
		r"Button\sA:\sX\+(\d+),\sY\+(\d+)\nButton\sB\:\sX\+(\d+),\sY\+(\d+)\nPrize:\sX=(\d+),\sY=(\d+)",
	)
	.unwrap();
	let captures = re.captures_iter(input);
	let mut total: u32 = 0;
	for c in captures {
		let a_x = c.get(1).unwrap().as_str().parse::<u32>().unwrap();
		let a_y = c.get(2).unwrap().as_str().parse::<u32>().unwrap();
		let b_x = c.get(3).unwrap().as_str().parse::<u32>().unwrap();
		let b_y = c.get(4).unwrap().as_str().parse::<u32>().unwrap();
		let x_target = c.get(5).unwrap().as_str().parse::<u32>().unwrap();
		let y_target = c.get(6).unwrap().as_str().parse::<u32>().unwrap();

		let mut min = u32::MAX;
		for a in 0..=100 {
			for b in 0..=100 {
				let x = a_x * a + b_x * b;
				let y = a_y * a + b_y * b;
				if x == x_target && y == y_target {
					//println!("A: {}, B: {}", a, b);
					let tokens = a * 3 + b;
					if tokens < min {
						min = tokens;
					}
				}
			}
		}

		if min != u32::MAX {
			total += min;
		}
	}
	total.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
			.to_string();
		let result = invoke(&input);
		assert_eq!(result, "480");
	}
}
