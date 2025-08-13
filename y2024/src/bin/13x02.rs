use ndarray::prelude::*;
use ndarray_linalg::Solve;
use regex::Regex;

// Classic
// Part 1: Brute Force It. Part 2. I know what's coming.
// Yes Part 2. Lets use maths!
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/01.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
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
    total.to_string()
}
