use itertools::Itertools;
use tokio::sync::mpsc::{self, Receiver, Sender};

#[tokio::main]
async fn main() {
    let input = include_str!("../../../puzzle_data/2019/07.txt");
    let out = invoke(input).await;
    println!("Best: {out}");
    // bench(invoke, input);
}

async fn invoke(input: &str) -> String {
    let memory: Vec<i64> = input
        .split(",")
        .map(|v| v.trim().parse::<i64>().unwrap())
        .collect();
    let mut best = 0;
    let phases: [i64; 5] = [5, 6, 7, 8, 9];
    for perm in phases.iter().permutations(phases.len()).unique() {
        let (send_a, recv_a) = mpsc::channel(10);
        let (send_b, recv_b) = mpsc::channel(10);
        let (send_c, recv_c) = mpsc::channel(10);
        let (send_d, recv_d) = mpsc::channel(10);
        let (send_e, recv_e) = mpsc::channel(10);

        let mut a = IntCodeComputer::new('A', recv_a, send_b.clone(), memory.clone());
        let mut b = IntCodeComputer::new('B', recv_b, send_c.clone(), memory.clone());
        let mut c = IntCodeComputer::new('C', recv_c, send_d.clone(), memory.clone());
        let mut d = IntCodeComputer::new('D', recv_d, send_e.clone(), memory.clone());
        let mut e = IntCodeComputer::new('E', recv_e, send_a.clone(), memory.clone());

        tokio::spawn(async move { a.run().await });
        tokio::spawn(async move { b.run().await });
        tokio::spawn(async move { c.run().await });
        tokio::spawn(async move { d.run().await });
        let e = tokio::spawn(async move { e.run().await });

        send_a.send(*perm[0]).await.unwrap();
        send_b.send(*perm[1]).await.unwrap();
        send_c.send(*perm[2]).await.unwrap();
        send_d.send(*perm[3]).await.unwrap();
        send_e.send(*perm[4]).await.unwrap();

        send_a.send(0).await.unwrap();

        let out = e.await.unwrap().unwrap();

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
    id: char,
    memory: Vec<i64>,
    ptr: usize,
    input: Receiver<i64>,
    out: i64,
    output: Sender<i64>,
}

impl IntCodeComputer {
    fn new(id: char, input: Receiver<i64>, output: Sender<i64>, memory: Vec<i64>) -> Self {
        Self {
            id,
            memory,
            ptr: 0,
            input,
            out: 0,
            output,
        }
    }

    async fn run(&mut self) -> Option<i64> {
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
                    //println!("[{}] waiting on input", self.id);
                    let val = self.input.recv().await.unwrap();
                    //println!("[{}] received {}", self.id, val);
                    let store = self.get_store_mut(1, Mode::Position);
                    *store = val;
                    self.ptr += 2;
                }
                OpCode::Output(m) => {
                    println!("[{}] sending output", self.id);
                    let val = self.get_val(1, m);
                    if self.output.send(val).await.is_err() {
                        println!("[{}] data not sent", self.id)
                    };
                    self.out = val;
                    self.ptr += 2;
                }
                OpCode::Halt => {
                    println!("[{}] halt", self.id);
                    return Some(self.out);
                }
            }
        }
        None
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
    use tokio::sync::mpsc;

    use super::*;

    #[tokio::test]
    async fn test_a() {
        println!("Testing A");
        let ins =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let memory: Vec<i64> = ins
            .split(",")
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();

        let (send_a, recv_a) = mpsc::channel(10);
        let (send_b, recv_b) = mpsc::channel(10);
        let (send_c, recv_c) = mpsc::channel(10);
        let (send_d, recv_d) = mpsc::channel(10);
        let (send_e, recv_e) = mpsc::channel(10);

        let mut a = IntCodeComputer::new('A', recv_a, send_b.clone(), memory.clone());
        let mut b = IntCodeComputer::new('B', recv_b, send_c.clone(), memory.clone());
        let mut c = IntCodeComputer::new('C', recv_c, send_d.clone(), memory.clone());
        let mut d = IntCodeComputer::new('D', recv_d, send_e.clone(), memory.clone());
        let mut e = IntCodeComputer::new('E', recv_e, send_a.clone(), memory.clone());

        tokio::spawn(async move { a.run().await });
        tokio::spawn(async move { b.run().await });
        tokio::spawn(async move { c.run().await });
        tokio::spawn(async move { d.run().await });
        let e = tokio::spawn(async move { e.run().await });

        send_a.send(9).await.unwrap();
        send_b.send(8).await.unwrap();
        send_c.send(7).await.unwrap();
        send_d.send(6).await.unwrap();
        send_e.send(5).await.unwrap();
        send_a.send(0).await.unwrap();
        let out = e.await.unwrap().unwrap();
        println!("Thruster Power: {}", out);
        assert_eq!(139629729, out);
    }

    #[tokio::test]
    async fn test_b() {
        println!("Testing A");
        let ins = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let memory: Vec<i64> = ins
            .split(",")
            .map(|v| v.trim().parse::<i64>().unwrap())
            .collect();

        let (send_a, recv_a) = mpsc::channel(10);
        let (send_b, recv_b) = mpsc::channel(10);
        let (send_c, recv_c) = mpsc::channel(10);
        let (send_d, recv_d) = mpsc::channel(10);
        let (send_e, recv_e) = mpsc::channel(10);

        let mut a = IntCodeComputer::new('A', recv_a, send_b.clone(), memory.clone());
        let mut b = IntCodeComputer::new('B', recv_b, send_c.clone(), memory.clone());
        let mut c = IntCodeComputer::new('C', recv_c, send_d.clone(), memory.clone());
        let mut d = IntCodeComputer::new('D', recv_d, send_e.clone(), memory.clone());
        let mut e = IntCodeComputer::new('E', recv_e, send_a.clone(), memory.clone());

        tokio::spawn(async move { a.run().await });
        tokio::spawn(async move { b.run().await });
        tokio::spawn(async move { c.run().await });
        tokio::spawn(async move { d.run().await });
        let e = tokio::spawn(async move { e.run().await });

        send_a.send(9).await.unwrap();
        send_b.send(7).await.unwrap();
        send_c.send(8).await.unwrap();
        send_d.send(5).await.unwrap();
        send_e.send(6).await.unwrap();
        send_a.send(0).await.unwrap();
        let out = e.await.unwrap().unwrap();
        println!("Thruster Power: {}", out);
        assert_eq!(18216, out);
    }
}
