#![allow(unused)]

fn main() {
    let input = include_str!("../../../puzzle_data/2019/09.txt");
    let out = invoke(input);
    println!("{out}");
    // bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let memory: Vec<i64> = input
        .trim()
        .split(",")
        .map(|v| v.trim().parse::<i64>().unwrap())
        .collect();
    let mut computer = IntCodeComputer::new(memory);
    println!("Run");
    computer.run();
    let out = computer.out;
    format!("{}", out)
}

#[derive(Debug)]
enum Mode {
    Immediate,
    Position,
    Relative,
}

impl From<char> for Mode {
    fn from(value: char) -> Self {
        match value {
            '0' => Mode::Position,
            '1' => Mode::Immediate,
            '2' => Mode::Relative,
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
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
    RelativeBase(Mode),
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
                let mode: Mode = Mode::from(chars[2]);
                OpCode::Output(mode)
            }
            "05" => {
                let mode1: Mode = Mode::from(chars[2]);
                let mode2: Mode = Mode::from(chars[1]);
                OpCode::JumpIfTrue(mode1, mode2)
            }
            "06" => {
                let mode1: Mode = Mode::from(chars[2]);
                let mode2: Mode = Mode::from(chars[1]);
                OpCode::JumpIfFalse(mode1, mode2)
            }
            "07" => {
                let mode1: Mode = Mode::from(chars[2]);
                let mode2: Mode = Mode::from(chars[1]);
                let mode3: Mode = Mode::from(chars[0]);
                OpCode::LessThan(mode1, mode2, mode3)
            }
            "08" => {
                let mode1: Mode = Mode::from(chars[2]);
                let mode2: Mode = Mode::from(chars[1]);
                let mode3: Mode = Mode::from(chars[0]);
                OpCode::Equals(mode1, mode2, mode3)
            }
            "09" => {
                let mode: Mode = Mode::from(chars[2]);
                OpCode::RelativeBase(mode)
            }
            "99" => OpCode::Halt,
            _ => {
                panic!("[OPCODE error] {opcode}")
            }
        }
    }
}

struct IntCodeComputer {
    relative_base: usize,
    memory: Vec<i64>,
    ptr: usize,
    out: i64,
}

impl IntCodeComputer {
    fn new(mut memory: Vec<i64>) -> Self {
        let mut extra_memory = vec![0; 1_000];
        memory.append(&mut extra_memory);
        Self {
            memory,
            ptr: 0,
            relative_base: 0,
            out: 0,
        }
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
                OpCode::JumpIfTrue(m1, m2) => {
                    if self.get_val(1, m1) > 0 {
                        self.ptr = self.get_val(2, m2) as usize;
                    } else {
                        self.ptr += 3;
                    }
                }
                OpCode::JumpIfFalse(m1, m2) => {
                    if self.get_val(1, m1) == 0 {
                        self.ptr = self.get_val(2, m2) as usize;
                    } else {
                        self.ptr += 3;
                    }
                }
                OpCode::LessThan(m1, m2, m3) => {
                    let val_a = self.get_val(1, m1);
                    let val_b = self.get_val(2, m2);
                    let store = self.get_store_mut(3, m3);
                    if val_a < val_b {
                        *store = 1;
                    } else {
                        *store = 0;
                    }
                    self.ptr += 4;
                }
                OpCode::Equals(m1, m2, m3) => {
                    let val_a = self.get_val(1, m1);
                    let val_b = self.get_val(2, m2);
                    let store = self.get_store_mut(3, m3);
                    if val_a == val_b {
                        *store = 1;
                    } else {
                        *store = 0;
                    }
                    self.ptr += 4;
                }
                OpCode::Input => {
                    let store = self.get_store_mut(1, Mode::Position);
                    // put into test
                    println!("INPUT");
                    *store = 1;
                    self.ptr += 2;
                }
                OpCode::Output(m) => {
                    let val = self.get_val(1, m);
                    self.out = val;
                    self.ptr += 2;
                }
                OpCode::Halt => {
                    println!("HALT");
                    break;
                }
                OpCode::RelativeBase(m) => {
                    let val = self.get_val(1, m);
                    // Check if goes negative.
                    let rel = self.relative_base as i64 + val;
                    println!("rel: {}", rel);
                    if rel < 0 {
                        panic!("Should not get here");
                    }
                    self.relative_base = rel as usize;
                }
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
            Mode::Relative => self.memory[self.relative_base],
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
            Mode::Relative => &mut self.memory[self.relative_base],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IntCodeComputer;

    #[test]
    fn test_a() {
        println!("Test A");
        let ins = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let memory: Vec<i64> = ins
            .split(",")
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();
        let mut computer = IntCodeComputer::new(memory);
        computer.run();
        let out = computer.out;
        let out = format!("{}", out);
        println!("{out}");
    }

    #[test]
    fn test_b() {
        println!("Test B");
        let ins = "1102,34915192,34915192,7,4,7,99,0";
        let memory: Vec<i64> = ins
            .split(",")
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();
        let mut computer = IntCodeComputer::new(memory);
        computer.run();
        let out = computer.out;
        let out = format!("{}", out);
        println!("{out}");
        assert_eq!(out.len(), 16);
    }

    #[test]
    fn test_c() {
        println!("Test C");
        let ins = "104,1125899906842624,99";
        let memory: Vec<i64> = ins
            .split(",")
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();
        let mut computer = IntCodeComputer::new(memory);
        computer.run();
        let out = computer.out;
        assert_eq!(out, 1125899906842624);
    }
}
