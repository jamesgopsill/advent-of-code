use itertools::Itertools;

fn main() {
    let input = include_str!("../../../puzzle_data/2019/07.txt");
    let out = invoke(input);
    println!("{out}");
    // bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let memory: Vec<i64> = input
        .split(",")
        .map(|v| v.trim().parse::<i64>().unwrap())
        .collect();
    let mut best = 0;
    let phases: [i64; 5] = [0, 1, 2, 3, 4];
    for perm in phases.iter().permutations(phases.len()).unique() {
        let mut out: i64 = 0;
        for p in perm {
            let mut computer = IntCodeComputer::new(*p, out, memory.clone());
            computer.run();
            out = computer.out;
        }
        if out > best {
            best = out;
        }
    }
    best.to_string()
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
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode, Mode),
    Equals(Mode, Mode, Mode),
}

impl From<&str> for OpCode {
    fn from(value: &str) -> Self {
        if value.len() != 5 {
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
            "99" => OpCode::Halt,
            _ => {
                panic!("[OPCODE error] {opcode}")
            }
        }
    }
}

struct IntCodeComputer {
    inps: Vec<i64>,
    memory: Vec<i64>,
    ptr: usize,
    out: i64,
}

impl IntCodeComputer {
    fn new(phase: i64, input: i64, memory: Vec<i64>) -> Self {
        Self {
            inps: vec![input, phase],
            memory,
            ptr: 0,
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
                    let inp = self.inps.pop().unwrap();
                    let store = self.get_store_mut(1, Mode::Position);
                    *store = inp;
                    self.ptr += 2;
                }
                OpCode::Output(m) => {
                    let val = self.get_val(1, m);
                    self.out = val;
                    self.ptr += 2;
                }
                OpCode::Halt => {
                    //println!("HALT");
                    break;
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
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let ins = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let memory: Vec<i64> = ins
            .split(",")
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();
        let mut out: i64 = 0;
        let phases: [i64; 5] = [4, 3, 2, 1, 0];
        for p in phases {
            let mut computer = IntCodeComputer::new(p, out, memory.clone());
            computer.run();
            out = computer.out;
            println!("{}", out);
        }
        assert_eq!(43210, out);
    }

    #[test]
    fn test_b() {
        let ins = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        let memory: Vec<i64> = ins
            .split(",")
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();
        let mut out: i64 = 0;
        let phases: [i64; 5] = [0, 1, 2, 3, 4];
        for p in phases {
            let mut computer = IntCodeComputer::new(p, out, memory.clone());
            computer.run();
            out = computer.out;
            println!("{}", out);
        }
        assert_eq!(54321, out);
    }

    #[test]
    fn test_c() {
        let ins = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        let memory: Vec<i64> = ins
            .split(",")
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();
        let mut out: i64 = 0;
        let phases: [i64; 5] = [1, 0, 4, 3, 2];
        for p in phases {
            let mut computer = IntCodeComputer::new(p, out, memory.clone());
            computer.run();
            out = computer.out;
            println!("{}", out);
        }
        assert_eq!(65210, out);
    }
}
