use ndarray::prelude::*;
use ndarray_linalg::Solve;
use regex::Regex;

// Classic
// Part 1: Brute Force It. Part 2. I know what's coming.
// Yes Part 2. Lets do use maths!
pub fn invoke(input: &String) -> u64 {
	let re = Regex::new(
		r"Button\sA:\sX\+(\d+),\sY\+(\d+)\nButton\sB\:\sX\+(\d+),\sY\+(\d+)\nPrize:\sX=(\d+),\sY=(\d+)",
	)
	.unwrap();
	let captures = re.captures_iter(input);
	let mut total: u64 = 0;
	let shift: f64 = 10_000_000_000_000.0;
	for c in captures {
		let a_x = c.get(1).unwrap().as_str().parse::<f64>().unwrap();
		let a_y = c.get(2).unwrap().as_str().parse::<f64>().unwrap();
		let b_x = c.get(3).unwrap().as_str().parse::<f64>().unwrap();
		let b_y = c.get(4).unwrap().as_str().parse::<f64>().unwrap();
		let mut x_target = c.get(5).unwrap().as_str().parse::<f64>().unwrap();
		x_target += shift;
		let mut y_target = c.get(6).unwrap().as_str().parse::<f64>().unwrap();
		y_target += shift;

		let u: Array2<f64> = array![[a_x, b_x], [a_y, b_y]];
		let v: Array1<f64> = array![x_target, y_target];
		let presses = u.solve(&v).unwrap();
		//println!("{}", presses);
		let a = presses[0].round() as u64;
		let b = presses[1].round() as u64;
		//println!("{} {}", a, b);
		let a_x = a_x as u64;
		let a_y = a_y as u64;
		let b_x = b_x as u64;
		let b_y = b_y as u64;
		let x = a_x * a + b_x * b;
		let y = a_y * a + b_y * b;
		let x_target = x_target as u64;
		let y_target = y_target as u64;
		if x == x_target && y == y_target {
			total += a * 3 + b;
		}
	}
	total
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
		assert_eq!(result, 480);
	}
}
