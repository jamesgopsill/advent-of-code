use regex::{Captures, Regex};
use std::{collections::HashSet, fs};
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2018/03.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut m = Matrix::new(1_000, 1_000);

    let mut final_id: u16 = 0;
    for line in input.lines() {
        let caps: Vec<Captures<'_>> = re.captures_iter(line).collect();
        let id = caps[0].get(0).unwrap().as_str().parse::<u16>().unwrap();
        let row = caps[1].get(0).unwrap().as_str().parse::<usize>().unwrap();
        let col = caps[2].get(0).unwrap().as_str().parse::<usize>().unwrap();
        let width = caps[3].get(0).unwrap().as_str().parse::<usize>().unwrap();
        let height = caps[4].get(0).unwrap().as_str().parse::<usize>().unwrap();
        m.add_claim(id, row, col, width, height);
        final_id = id;
    }

    let idx = m.no_overlap(final_id);
    idx.to_string()
}

struct Matrix {
    cells: Vec<Vec<u16>>,
    rows: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut cells: Vec<Vec<u16>> = Vec::with_capacity(rows * cols);
        for _ in 0..rows * cols {
            cells.push(Vec::new());
        }
        Self { cells, rows }
    }

    fn two_dim_to_one_dim(&self, row: usize, col: usize) -> usize {
        (row * self.rows) + col
    }

    fn add_claim(&mut self, id: u16, row: usize, col: usize, width: usize, height: usize) {
        for r in row..row + width {
            for c in col..col + height {
                let idx = self.two_dim_to_one_dim(r, c);
                self.cells[idx].push(id);
            }
        }
    }

    fn no_overlap(&self, max_id: u16) -> u16 {
        let mut overlapping_ids: HashSet<u16> = HashSet::new();
        for cell in &self.cells {
            if cell.len() > 1 {
                for id in cell {
                    overlapping_ids.insert(*id);
                }
            }
        }
        for idx in 1..=max_id {
            if !overlapping_ids.contains(&idx) {
                return idx;
            }
        }
        0
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
        assert_eq!(result, "3");
    }
}
