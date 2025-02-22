#![allow(dead_code)]
//use rayon::prelude::*;
// Knowing there is a better way but lets try brute. :D
use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/17.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(_input: &str) -> String {
	//let mut lines = input.lines();
	//let program = lines.nth(4).unwrap().split(":").last().unwrap();
	//let program = program
	//.split(",")
	//.collect::<Vec<&str>>()
	//.iter()
	//.map(|f| f.trim().parse::<u32>().unwrap())
	//.collect::<Vec<u32>>();
	//let mut computer = Computer::new(0, 0, 0, program);
	/*

	(0..u32::MAX).into_par_iter().for_each(|i| {
		let computer = Computer::new(
			i,
			0,
			0,
			vec![2, 4, 1, 1, 7, 5, 1, 5, 4, 1, 5, 5, 0, 3, 3, 0],
		);
		if computer.program == computer.out {
			println!("{}", i.to_string());
		}
	});

	for i in 0..u32::MAX {
		if i % 100_000 == 0 {
			println!("{}", i);
		}
		computer.reset();
		computer.a = i;
		computer.run();
		if computer.program == computer.out {
			return i.to_string();
		}
	}
	*/
	"".to_string()
}

#[derive(Debug)]
struct Computer {
	a: u32,
	b: u32,
	c: u32,
	pointer: usize,
	program: Vec<u32>,
	out: Vec<u32>,
}

impl Computer {
	fn new(
		a: u32,
		b: u32,
		c: u32,
		program: Vec<u32>,
	) -> Self {
		Self {
			a,
			b,
			c,
			pointer: 0,
			program,
			out: vec![],
		}
	}

	fn reset(&mut self) {
		self.pointer = 0;
		self.a = 0;
		self.b = 0;
		self.c = 0;
		self.out.clear();
	}

	fn run(&mut self) {
		while self.pointer < self.program.len() && self.program.len() >= self.out.len() {
			self.compute();
		}
	}

	fn compute(&mut self) {
		let opcode = *self.program.get(self.pointer).unwrap();
		let literal_operand = *self.program.get(self.pointer + 1).unwrap();
		let mut combo_operand: u32 = 0;
		match literal_operand {
			0..=3 => combo_operand = literal_operand,
			4 => combo_operand = self.a,
			5 => combo_operand = self.b,
			6 => combo_operand = self.c,
			_ => {}
		}
		//println!("{} {} {}", opcode, literal_operand, combo_operand);
		self.pointer += 2;
		match opcode {
			0 => {
				let numerator = self.a;
				let denominator = 2_u32.pow(combo_operand);
				self.a = numerator / denominator;
			}
			1 => self.b ^= literal_operand, //= self.b ^ literal_operand,
			2 => self.b = combo_operand % 8,
			3 => match self.a {
				0 => {}
				_ => self.pointer = literal_operand as usize,
			},
			4 => self.b ^= self.c, // = self.b ^ self.c,
			5 => self.out.push(combo_operand % 8),
			6 => {
				let numerator = self.a;
				let denominator = 2_u32.pow(combo_operand);
				self.b = numerator / denominator;
			}
			7 => {
				let numerator = self.a;
				let denominator = 2_u32.pow(combo_operand);
				self.c = numerator / denominator;
			}
			_ => {}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
	";
		let result = invoke(input);
		assert_eq!(result, "117440");
	}
}
