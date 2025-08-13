use regex::{Captures, Regex};
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2018/03.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    // define the regex to identify all the numbers on a line.
    let re = Regex::new(r"(\d+)").unwrap();
    // create an instance of the sheet.
    let mut sheet = Sheet::new(1_000, 1_000);

    // for each line representing a claim
    for line in input.lines() {
        // Capture all the values
        let caps: Vec<Captures<'_>> = re.captures_iter(line).collect();
        // Parse them all.
        let id = caps[0].get(0).unwrap().as_str().parse::<u16>().unwrap();
        let row = caps[1].get(0).unwrap().as_str().parse::<usize>().unwrap();
        let col = caps[2].get(0).unwrap().as_str().parse::<usize>().unwrap();
        let width = caps[3].get(0).unwrap().as_str().parse::<usize>().unwrap();
        let height = caps[4].get(0).unwrap().as_str().parse::<usize>().unwrap();
        // Add the claim to the sheet.
        sheet.add_claim(id, row, col, width, height);
    }

    // Identify the number of overlaps on the sheet.
    let overlaps = sheet.overlaps();
    overlaps.to_string()
}

/// A struct to represent the sheet of fabric (a matrix).
struct Sheet {
    cells: Vec<Vec<u16>>,
    rows: usize,
}

impl Sheet {
    // Create a new instance of sheet
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut cells: Vec<Vec<u16>> = Vec::with_capacity(rows * cols);
        for _ in 0..rows * cols {
            cells.push(Vec::new());
        }
        Self { cells, rows }
    }

    // Convert a 2D value into a 1D value.
    fn two_dim_to_one_dim(&self, row: usize, col: usize) -> usize {
        (row * self.rows) + col
    }

    // Log a claim against the sheet.
    fn add_claim(&mut self, id: u16, row: usize, col: usize, width: usize, height: usize) {
        for r in row..row + width {
            for c in col..col + height {
                let idx = self.two_dim_to_one_dim(r, c);
                self.cells[idx].push(id);
            }
        }
    }

    // Detect the overlaps by iterating over the cells and identifying ones with multiple claims.
    fn overlaps(&self) -> u32 {
        let mut overlaps: u32 = 0;
        for cell in &self.cells {
            if cell.len() > 1 {
                overlaps += 1;
            }
        }
        overlaps
    }
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";
        let result = invoke(input);
        assert_eq!(result, "4");
    }
}
