use itertools::Itertools;
use std::{collections::HashMap, fs};
//use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2017/08.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	//bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut memory: HashMap<Box<str>, i64> = HashMap::new();
	let mut max = i64::MIN;
	for line in input.lines() {
		let ins = Instruction::try_from(line).unwrap();
		ins.execute(&mut memory);
		for v in memory.values() {
			if *v > max {
				max = *v;
			}
		}
	}
	max.to_string()
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
";
		let result = invoke(input);
		assert_eq!(result, "10");
	}
}

#[derive(Debug)]
enum OperationKind {
	Increment,
	Decrement,
}

#[derive(Debug)]
enum ConditionKind {
	Equal,
	GreaterThan,
	GreaterThanOrEqualTo,
	LessThan,
	LessThenOrEqualTo,
	NotEqual,
}

struct Instruction {
	opreg: Box<str>,
	opval: i64,
	opkind: OperationKind,
	coreg: Box<str>,
	coval: i64,
	cokind: ConditionKind,
}

impl TryFrom<&str> for Instruction {
	type Error = &'static str;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		let els = s.split_whitespace().collect_vec();
		let opreg = els[0].to_owned().into_boxed_str();
		let opkind = match els[1] {
			"inc" => Ok(OperationKind::Increment),
			"dec" => Ok(OperationKind::Decrement),
			_ => Err("No Operation Found"),
		}
		.unwrap();
		let opval = els[2].parse::<i64>().unwrap();

		let coreg = els[4].to_owned().into_boxed_str();
		let cokind = match els[5] {
			"==" => Ok(ConditionKind::Equal),
			">" => Ok(ConditionKind::GreaterThan),
			">=" => Ok(ConditionKind::GreaterThanOrEqualTo),
			"!=" => Ok(ConditionKind::NotEqual),
			"<" => Ok(ConditionKind::LessThan),
			"<=" => Ok(ConditionKind::LessThenOrEqualTo),
			_ => Err("No Condition Found"),
		}
		.unwrap();
		let coval = els[6].parse::<i64>().unwrap();

		Ok(Self {
			opreg,
			opval,
			opkind,
			coreg,
			coval,
			cokind,
		})
	}
}

impl Instruction {
	fn execute(
		&self,
		memory: &mut HashMap<Box<str>, i64>,
	) {
		if self.condition_satisified(memory) {
			self.operate(memory);
		};
	}

	fn condition_satisified(
		&self,
		memory: &HashMap<Box<str>, i64>,
	) -> bool {
		let regval: i64 = match memory.get(&self.coreg) {
			Some(v) => *v,
			None => 0,
		};
		match self.cokind {
			ConditionKind::Equal => regval == self.coval,
			ConditionKind::GreaterThan => regval > self.coval,
			ConditionKind::GreaterThanOrEqualTo => regval >= self.coval,
			ConditionKind::NotEqual => regval != self.coval,
			ConditionKind::LessThan => regval < self.coval,
			ConditionKind::LessThenOrEqualTo => regval <= self.coval,
		}
	}

	fn operate(
		&self,
		memory: &mut HashMap<Box<str>, i64>,
	) {
		let regval = memory.get_mut(&self.opreg);
		if let Some(val) = regval {
			match self.opkind {
				OperationKind::Decrement => *val -= self.opval,
				OperationKind::Increment => *val += self.opval,
			}
		} else {
			// Register a new value;
			match self.opkind {
				OperationKind::Decrement => {
					memory.insert(self.opreg.clone(), -self.opval);
				}
				OperationKind::Increment => {
					memory.insert(self.opreg.clone(), self.opval);
				}
			}
		}
	}
}
