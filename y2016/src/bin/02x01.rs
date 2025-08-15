use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2016/02.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let lines = input.lines();
    let mut p = PinPad::new();
    let mut passcode: String = String::new();
    for line in lines {
        for c in line.chars() {
            p.step(c);
        }
        let code = p.value.to_string();
        passcode.push_str(code.as_str());
    }
    passcode
}

struct PinPad {
    value: u32,
}

impl PinPad {
    fn new() -> Self {
        Self { value: 5 }
    }

    fn step(&mut self, c: char) {
        match c {
            'U' => {
                if let 4..10 = self.value {
                    self.value -= 3
                }
            }
            'R' => match self.value {
                1..3 => self.value += 1,
                3 => {}
                4..6 => self.value += 1,
                6 => {}
                7..9 => self.value += 1,
                _ => {}
            },
            'D' => {
                if let 1..7 = self.value {
                    self.value += 3
                }
            }
            'L' => match self.value {
                2..4 => self.value -= 1,
                4 => {}
                5..7 => self.value -= 1,
                7 => {}
                8..10 => self.value -= 1,
                _ => {}
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let result = invoke(
            "ULL
RRDDD
LURDL
UUUUD
",
        );
        assert_eq!(result, "1985");
    }
}
