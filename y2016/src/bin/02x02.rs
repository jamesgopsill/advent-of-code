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
        match p.pin {
            Pin::One => passcode.push('1'),
            Pin::Two => passcode.push('2'),
            Pin::Three => passcode.push('3'),
            Pin::Four => passcode.push('4'),
            Pin::Five => passcode.push('5'),
            Pin::Six => passcode.push('6'),
            Pin::Seven => passcode.push('7'),
            Pin::Eight => passcode.push('8'),
            Pin::Nine => passcode.push('9'),
            Pin::A => passcode.push('A'),
            Pin::B => passcode.push('B'),
            Pin::C => passcode.push('C'),
            Pin::D => passcode.push('D'),
        }
    }
    passcode
}

enum Pin {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

struct PinPad {
    pin: Pin,
}

impl PinPad {
    fn new() -> Self {
        Self { pin: Pin::Five }
    }

    fn step(&mut self, c: char) {
        match self.pin {
            Pin::One => if c == 'D' { self.pin = Pin::Three },
            Pin::Two => match c {
                'R' => self.pin = Pin::Three,
                'D' => self.pin = Pin::Six,
                _ => {}
            },
            Pin::Three => match c {
                'R' => self.pin = Pin::Four,
                'L' => self.pin = Pin::Two,
                'D' => self.pin = Pin::Seven,
                'U' => self.pin = Pin::One,
                _ => {}
            },
            Pin::Four => match c {
                'L' => self.pin = Pin::Three,
                'D' => self.pin = Pin::Eight,
                _ => {}
            },
            Pin::Five => if c == 'R' { self.pin = Pin::Six },
            Pin::Six => match c {
                'R' => self.pin = Pin::Seven,
                'L' => self.pin = Pin::Five,
                'D' => self.pin = Pin::A,
                'U' => self.pin = Pin::Two,
                _ => {}
            },
            Pin::Seven => match c {
                'R' => self.pin = Pin::Eight,
                'L' => self.pin = Pin::Six,
                'D' => self.pin = Pin::B,
                'U' => self.pin = Pin::Three,
                _ => {}
            },
            Pin::Eight => match c {
                'R' => self.pin = Pin::Nine,
                'L' => self.pin = Pin::Seven,
                'D' => self.pin = Pin::C,
                'U' => self.pin = Pin::Four,
                _ => {}
            },
            Pin::Nine => if c == 'L' { self.pin = Pin::Eight },
            Pin::A => match c {
                'R' => self.pin = Pin::B,
                'U' => self.pin = Pin::Six,
                _ => {}
            },
            Pin::B => match c {
                'R' => self.pin = Pin::C,
                'L' => self.pin = Pin::A,
                'D' => self.pin = Pin::D,
                'U' => self.pin = Pin::Seven,
                _ => {}
            },
            Pin::C => match c {
                'L' => self.pin = Pin::B,
                'U' => self.pin = Pin::Eight,
                _ => {}
            },
            Pin::D => if c == 'U' { self.pin = Pin::B },
        };
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
        assert_eq!(result, "5DB3");
    }
}
