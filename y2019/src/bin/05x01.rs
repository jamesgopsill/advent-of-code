#![allow(unused)]
use std::{io::Error, os::unix::net::Incoming, process::id};

use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2019/05.txt");
    let out = invoke(input);
    println!("{out}");
    // bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let memory: Vec<i64> = input
        .split(",")
        .map(|v| v.trim().parse::<i64>().unwrap())
        .collect();
    let mut computer = IntCodeComputer::new(memory);
    computer.run();
    "[FINISHED]".to_string()
}

#[derive(Debug)]
enum Mode {
    Immediate,
    Position,
}

impl From<char> for Mode {
    fn from(value: char) -> Self {
        match value {
            '0' => Mode::Position,
            '1' => Mode::Immediate,
            _ => panic!("Incorrect Mode"),
        }
    }
}

#[derive(Debug)]
enum OpCode {
    Add(Mode, Mode, Mode),
    Mul(Mode, Mode, Mode),
    Halt,
    Input,
    Output(Mode),
}

impl From<&str> for OpCode {
    fn from(value: &str) -> Self {
        if (value.len() != 5) {
            panic!("Incorrect str length");
        }
        let opcode: &str = &value[3..];
        let chars: Vec<char> = value.chars().collect();
        match opcode {
            "01" => {
                let mode1: Mode = Mode::from(chars[2]);
                let mode2: Mode = Mode::from(chars[1]);
                let mode3: Mode = Mode::from(chars[0]);
                OpCode::Add(mode1, mode2, mode3)
            }
            "02" => {
                let mode1: Mode = Mode::from(chars[2]);
                let mode2: Mode = Mode::from(chars[1]);
                let mode3: Mode = Mode::from(chars[0]);
                OpCode::Mul(mode1, mode2, mode3)
            }
            "03" => OpCode::Input,
            "04" => {
                let mode1: Mode = match chars[2] {
                    '0' => Mode::Position,
                    '1' => Mode::Immediate,
                    _ => panic!("Incorrect Mode"),
                };
                OpCode::Output(mode1)
            }
            "99" => OpCode::Halt,
            _ => {
                panic!("[OPCODE error] {opcode}")
            }
        }
    }
}

struct IntCodeComputer {
    memory: Vec<i64>,
    ptr: usize,
}

impl IntCodeComputer {
    fn new(memory: Vec<i64>) -> Self {
        Self { memory, ptr: 0 }
    }

    fn run(&mut self) {
        let ptr_max = self.memory.len();
        while self.ptr < ptr_max {
            // Pad out the code.
            let opcode = format!("{:0>5}", self.memory[self.ptr]);
            let opcode = OpCode::from(opcode.as_str());
            match opcode {
                OpCode::Add(m1, m2, m3) => {
                    let val_a = self.get_val(1, m1);
                    let val_b = self.get_val(2, m2);
                    let store = self.get_store_mut(3, m3);
                    *store = val_a + val_b;
                    self.ptr += 4;
                }
                OpCode::Mul(m1, m2, m3) => {
                    let val_a = self.get_val(1, m1);
                    let val_b = self.get_val(2, m2);
                    let store = self.get_store_mut(3, m3);
                    *store = val_a * val_b;
                    self.ptr += 4;
                }
                OpCode::Input => {
                    let store = self.get_store_mut(1, Mode::Position);
                    *store = 1;
                    self.ptr += 2;
                }
                OpCode::Output(m) => {
                    let val = self.get_val(1, m);
                    if val > 0 {
                        println!("[FAIL]: {val}");
                    } else {
                        println!("[PASS]: {val}");
                    }
                    self.ptr += 2;
                }
                OpCode::Halt => {
                    println!("HALT");
                    break;
                }
                _ => {}
            }
        }
    }

    fn get_val(&self, inc: usize, mode: Mode) -> i64 {
        match mode {
            Mode::Immediate => self.memory[self.ptr + inc],
            Mode::Position => {
                let idx = self.memory[self.ptr + inc] as usize;
                self.memory[idx]
            }
        }
    }

    /// Should return the address to store the new value. Should always be in position mode.
    fn get_store_mut(&mut self, inc: usize, mode: Mode) -> &mut i64 {
        match mode {
            Mode::Immediate => panic!("Should be Position for storing"),
            Mode::Position => {
                let idx = self.memory[self.ptr + inc] as usize;
                &mut self.memory[idx]
            }
        }
    }
}

#[cfg(test)]
mod tests {}
