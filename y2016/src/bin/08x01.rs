use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle_data/2016/08.txt").unwrap();
    let out = invoke(&input, 50, 6);
    println!("{}", out);
}

fn invoke(
    input: &str,
    rows: usize,
    cols: usize,
) -> String {
    let mut display = Display::new(rows, cols);
    let digits = Regex::new(r"\d+").unwrap();
    display.print();
    for line in input.lines() {
        let mut digits = digits.captures_iter(line);
        let digit_one = digits
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let digit_two = digits
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        //println!("{} {}", digit_one, digit_two);

        if line.starts_with("rect") {
            display.rect(digit_two, digit_one);
            //println!("----");
            //display.print();
            continue;
        }

        if line.starts_with("rotate column") {
            display.rotate_col(digit_one, digit_two);
            //println!("----");
            //display.print();
            continue;
        }

        if line.starts_with("rotate row") {
            display.rotate_row(digit_one, digit_two);
            //println!("----");
            //display.print();
            continue;
        }
    }

    println!("----");
    display.print();

    let mut ans: u32 = 0;
    for row in display.0 {
        for led in row {
            if led == '#' {
                ans += 1;
            }
        }
    }
    ans.to_string()
}

struct Display(Vec<Vec<char>>);

impl Display {
    fn new(
        rows: usize,
        cols: usize,
    ) -> Self {
        let mut mat = vec![];
        for _ in 0..rows {
            let mut row = vec![];
            for _ in 0..cols {
                row.push('.');
            }
            mat.push(row);
        }
        Display(mat)
    }

    fn rect(
        &mut self,
        rows: usize,
        cols: usize,
    ) {
        for row in 0..rows {
            for col in 0..cols {
                self.0[row][col] = '#'
            }
        }
    }

    fn rotate_row(
        &mut self,
        row: usize,
        n: usize,
    ) {
        self.0[row].rotate_right(n);
    }

    fn rotate_col(
        &mut self,
        col: usize,
        n: usize,
    ) {
        let mut tmp = vec![];
        for row in self.0.iter() {
            tmp.push(row[col]);
        }
        tmp.rotate_right(n);
        for (i, c) in tmp.iter().enumerate() {
            self.0[i][col] = c.clone();
        }
    }

    fn print(&self) {
        for row in self.0.iter() {
            for c in row {
                print!("{}", c);
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1"
            .to_string();
        let result = invoke(&input, 3, 7);
        assert_eq!(result, "6");
    }
}
