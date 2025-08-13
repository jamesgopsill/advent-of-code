use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/11.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    // Convert the image as a Vec<Vec<char>>
    let lines: Vec<&str> = input.lines().collect();
    let mut image: Vec<Vec<char>> = vec![];
    for line in lines.iter() {
        let mut row = vec![];
        for char in line.chars() {
            row.push(char);
        }
        image.push(row);
    }

    // Create the universe
    let mut universe = Universe::new(image);

    universe.print_image();

    universe.expand();

    universe.print_image();

    universe.identify_galaxies();
    println!("{:?}", universe.galaxies);

    let mut sum = 0;
    let n_galaxies = universe.galaxies.len();
    for i in 0..n_galaxies {
        for j in (i + 1)..n_galaxies {
            let row_diff = (universe.galaxies[i].0 - universe.galaxies[j].0).abs();
            let col_diff = (universe.galaxies[i].1 - universe.galaxies[j].1).abs();
            sum += row_diff;
            sum += col_diff;
        }
    }

    sum.to_string()
}

struct Universe {
    pub image: Vec<Vec<char>>,
    pub galaxies: Vec<(i32, i32)>,
}

impl Universe {
    pub fn new(image: Vec<Vec<char>>) -> Self {
        Self {
            image,
            galaxies: vec![],
        }
    }

    pub fn print_image(&self) {
        for row in &self.image {
            for char in row {
                print!("{char}");
            }
            println!();
        }
    }

    pub fn expand(&mut self) {
        self.expand_rows();
        self.expand_cols();
    }

    pub fn expand_rows(&mut self) {
        let mut idxs: Vec<usize> = vec![];
        for (i, row) in self.image.iter().enumerate() {
            if !row.contains(&'#') {
                idxs.push(i);
            }
        }
        idxs.reverse();
        for idx in idxs {
            let row = self.image[idx].clone();
            self.image.insert(idx, row);
        }
    }

    pub fn expand_cols(&mut self) {
        let n_rows = self.image.len();
        let n_cols = self.image[0].len();

        let mut idxs: Vec<usize> = vec![];
        for i in 0..n_cols {
            let mut empty_space = true;
            for j in 0..n_rows {
                if self.image[j][i] == '#' {
                    empty_space = false;
                    break;
                }
            }
            if empty_space {
                idxs.push(i);
            }
        }
        idxs.reverse();
        for idx in idxs {
            for row in self.image.iter_mut() {
                row.insert(idx, '.');
            }
        }
    }

    pub fn identify_galaxies(&mut self) {
        for (row, chars) in self.image.iter().enumerate() {
            for (col, char) in chars.iter().enumerate() {
                if *char == '#' {
                    self.galaxies.push((row as i32, col as i32));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let result = invoke(input);
        assert_eq!(result, "374");
    }
}
