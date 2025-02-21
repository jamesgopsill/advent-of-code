use itertools::Itertools;
use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2024/17.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut lines = input.lines();
	let register_a = lines
		.next()
		.unwrap()
		.split_whitespace()
		.last()
		.unwrap()
		.parse::<u32>()
		.unwrap();
	//println!("Register A: {}", register_a);
	let register_b = lines
		.next()
		.unwrap()
		.split_whitespace()
		.last()
		.unwrap()
		.parse::<u32>()
		.unwrap();
	//println!("Register B: {}", register_b);
	let register_c = lines
		.next()
		.unwrap()
		.split_whitespace()
		.last()
		.unwrap()
		.parse::<u32>()
		.unwrap();
	//println!("Register C: {}", register_c);
	lines.next();
	let program = lines.next().unwrap().split(":").last().unwrap();
	let program = program
		.split(",")
		.collect::<Vec<&str>>()
		.iter()
		.map(|f| f.trim().parse::<u32>().unwrap())
		.collect::<Vec<u32>>();
	let mut computer = Computer::new(register_a, register_b, register_c, program);
	println!("{:?}", computer);
	computer.run();
	println!("{:?}", computer);
	let joined = Itertools::join(&mut computer.out.iter(), ",");
	joined
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

	fn run(&mut self) {
		while self.pointer < self.program.len() {
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
			1 => self.b = self.b ^ literal_operand,
			2 => self.b = combo_operand % 8,
			3 => match self.a {
				0 => {}
				_ => self.pointer = literal_operand as usize,
			},
			4 => self.b = self.b ^ self.c,
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
mod tests_17x01 {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "Register A: 0
	Register B: 0
	Register C: 9

	Program: 2,6
	";
		let result = invoke(input);
		assert_eq!(result, "");
	}

	#[test]
	fn test_b() {
		let input = "Register A: 10
	Register B: 0
	Register C: 0

	Program: 5,0,5,1,5,4
	";
		let result = invoke(input);
		assert_eq!(result, "0,1,2");
	}

	#[test]
	fn test_c() {
		let input = "Register A: 2024
	Register B: 0
	Register C: 0

	Program: 0,1,5,4,3,0
	";
		let result = invoke(input);
		assert_eq!(result, "4,2,5,6,7,7,7,7,3,1,0");
	}

	#[test]
	fn test_d() {
		let input = "Register A: 0
	Register B: 29
	Register C: 0

	Program: 1,7
	";
		let result = invoke(input);
		assert_eq!(result, "");
	}

	#[test]
	fn test_e() {
		let input = "Register A: 0
	Register B: 2024
	Register C: 43690

	Program: 4,0
	";
		let result = invoke(input);
		assert_eq!(result, "");
	}
}
