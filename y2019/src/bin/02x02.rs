#![allow(unused)]
use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2019/02.txt");
    let out = invoke(input);
    println!("{out}");
    //bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let memory: Vec<usize> = input
        .split(",")
        .map(|v| v.trim().parse::<usize>().unwrap())
        .collect();
    let mut out = 0;
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut computer = IntCodeComputer::new(memory.clone());
            computer.memory[1] = noun;
            computer.memory[2] = verb;
            computer.run();
            if computer.memory[0] == 19690720 {
                out = 100 * noun + verb;
            }
        }
    }
    out.to_string()
}

struct IntCodeComputer {
    memory: Vec<usize>,
    ptr: usize,
}

impl IntCodeComputer {
    fn new(memory: Vec<usize>) -> Self {
        Self { memory, ptr: 0 }
    }

    fn run(&mut self) {
        let ptr_max = self.memory.len();
        while self.ptr < ptr_max {
            let instruction = Instruction::from_usize(self.memory[self.ptr]);
            match instruction {
                Some(Instruction::Add) => {
                    let addr_a = self.memory[self.ptr + 1];
                    let addr_b = self.memory[self.ptr + 2];
                    let addr_store = self.memory[self.ptr + 3];
                    self.memory[addr_store] = self.memory[addr_a] + self.memory[addr_b];
                    self.ptr += 4;
                }
                Some(Instruction::Multiply) => {
                    let addr_a = self.memory[self.ptr + 1];
                    let addr_b = self.memory[self.ptr + 2];
                    let addr_store = self.memory[self.ptr + 3];
                    self.memory[addr_store] = self.memory[addr_a] * self.memory[addr_b];
                    self.ptr += 4
                }
                Some(Instruction::Halt) => break,
                None => {
                    panic!("Should not get here")
                }
            }
        }
    }
}

enum Instruction {
    Add,
    Multiply,
    Halt,
}

impl Instruction {
    fn from_usize(val: usize) -> Option<Self> {
        match val {
            1 => Some(Instruction::Add),
            2 => Some(Instruction::Multiply),
            99 => Some(Instruction::Halt),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IntCodeComputer;

    #[test]
    fn test_a() {
        let input = "1,0,0,0,99";
        let items: Vec<usize> = input
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let mut computer = IntCodeComputer::new(items);
        computer.run();
        println!("{:?}", computer.memory);
        assert_eq!(computer.memory[0], 2);
    }

    #[test]
    fn test_b() {
        let input = "2,3,0,3,99";
        let items: Vec<usize> = input
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let mut computer = IntCodeComputer::new(items);
        computer.run();
        assert_eq!(computer.memory[0], 2);
    }

    #[test]
    fn test_c() {
        let input = "2,4,4,5,99,0";
        let items: Vec<usize> = input
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let mut computer = IntCodeComputer::new(items);
        computer.run();
        assert_eq!(computer.memory[0], 2);
    }

    #[test]
    fn test_d() {
        let input = "1,1,1,4,99,5,6,0,99";
        let items: Vec<usize> = input
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let mut computer = IntCodeComputer::new(items);
        computer.run();
        assert_eq!(computer.memory[0], 30);
    }
}
