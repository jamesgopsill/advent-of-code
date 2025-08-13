use core::panic;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/10.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

// TODO: Not FINISHED
fn invoke(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut map: Vec<Vec<char>> = vec![];

    // Step 1. Create the map
    for line in lines.iter() {
        let mut r = vec![];
        for char in line.chars() {
            r.push(char);
        }
        map.push(r);
    }

    let mut elf = Elf::new(map);
    elf.find_start();

    // println!("{:?}", elf.current_tile);

    while elf.step() {}

    // Remove all the noise and leaving just the path
    // The elf is now a gardener
    elf.keep_the_path();

    elf.print_map();
    println!();

    // Work out what type the S pipe was
    // and replace it one the map
    elf.determine_s();

    elf.print_map();
    println!();

    // Moving top-left to bottom right
    // Classic enter exit the enclosing area of the pipe.
    let sum = elf.sum_enclosing_tiles();

    elf.print_map();

    // Return the result
    sum.to_string()
}

#[derive(Debug, PartialEq, Clone)]
struct Tile {
    row: usize,
    col: usize,
    char: char,
}

impl Tile {
    pub fn new(row: usize, col: usize, char: char) -> Self {
        Self { row, col, char }
    }
}

struct Elf {
    current_tile: Tile,
    past_tiles: Vec<Tile>,
    map: Vec<Vec<char>>,
}

impl Elf {
    pub fn new(map: Vec<Vec<char>>) -> Self {
        Self {
            current_tile: Tile::new(0, 0, '*'),
            past_tiles: vec![],
            map,
        }
    }

    pub fn find_start(&mut self) {
        for (row, chars) in self.map.iter().enumerate() {
            for (col, char) in chars.iter().enumerate() {
                if *char == 'S' {
                    self.current_tile = Tile::new(row, col, *char);
                }
            }
        }
    }

    pub fn step(&mut self) -> bool {
        match self.current_tile.char {
            'S' => {
                // Check all the possible valid positions
                if let Some(t) = self.tile_above() {
                    let chars = ['7', '|', 'F'];
                    if chars.contains(&t.char) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                if let Some(t) = self.tile_left() {
                    let chars = ['L', '-', 'F'];
                    if chars.contains(&t.char) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                if let Some(t) = self.tile_right() {
                    let chars = ['7', '-', 'J'];
                    if chars.contains(&t.char) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                if let Some(t) = self.tile_below() {
                    let chars = ['J', '|', 'L'];
                    if chars.contains(&t.char) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                panic!("Do not find a move from start");
            }
            '|' => {
                if let Some(t) = self.tile_above() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                if let Some(t) = self.tile_below() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                false
            }
            '-' => {
                if let Some(t) = self.tile_left() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                if let Some(t) = self.tile_right() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                false
            }
            'L' => {
                if let Some(t) = self.tile_right() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                if let Some(t) = self.tile_above() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                false
            }
            'J' => {
                if let Some(t) = self.tile_above() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                if let Some(t) = self.tile_left() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                false
            }
            '7' => {
                if let Some(t) = self.tile_left() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                if let Some(t) = self.tile_below() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                false
            }
            'F' => {
                if let Some(t) = self.tile_right() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                if let Some(t) = self.tile_below() {
                    if !self.past_tiles.contains(&t) {
                        self.past_tiles.push(self.current_tile.clone());
                        self.current_tile = t;
                        return true;
                    }
                }
                false
            }
            '.' => {
                panic!("No pipe here");
            }
            _ => {
                panic!("How did I get here!?");
            }
        }
    }

    pub fn tile_above(&self) -> Option<Tile> {
        if self.current_tile.row == 0 {
            return None;
        }
        let row = self.current_tile.row - 1;
        let col = self.current_tile.col;
        let char = self.map[row][col];
        Some(Tile::new(row, col, char))
    }

    pub fn tile_left(&self) -> Option<Tile> {
        if self.current_tile.col == 0 {
            return None;
        }
        let row = self.current_tile.row;
        let col = self.current_tile.col - 1;
        let char = self.map[row][col];
        Some(Tile::new(row, col, char))
    }

    pub fn tile_right(&self) -> Option<Tile> {
        if self.current_tile.col == self.map[0].len() - 1 {
            return None;
        }
        let row = self.current_tile.row;
        let col = self.current_tile.col + 1;
        let char = self.map[row][col];
        Some(Tile::new(row, col, char))
    }

    pub fn tile_below(&self) -> Option<Tile> {
        if self.current_tile.row == self.map.len() - 1 {
            return None;
        }
        let row = self.current_tile.row + 1;
        let col = self.current_tile.col;
        let char = self.map[row][col];
        Some(Tile::new(row, col, char))
    }

    pub fn keep_the_path(&mut self) {
        let tiles_to_keep = &mut self.past_tiles;
        tiles_to_keep.push(self.current_tile.clone());
        let n_rows = self.map.len();
        let n_cols = self.map[0].len();

        for i in 0..n_rows {
            for j in 0..n_cols {
                let tile = Tile::new(i, j, self.map[i][j]);
                if !tiles_to_keep.contains(&tile) {
                    self.map[i][j] = '~';
                }
            }
        }
    }

    pub fn determine_s(&mut self) {
        let s = self.past_tiles.first().unwrap();

        let mut left = '~';
        if s.col != 0 {
            left = self.map[s.row][s.col - 1];
        }

        let mut top = '~';
        if s.row != 0 {
            top = self.map[s.row - 1][s.col];
        }

        let mut right = '~';
        if s.col != self.map[0].len() - 1 {
            right = self.map[s.row][s.col + 1];
        }

        let mut bottom = '~';
        if s.row != self.map.len() - 1 {
            bottom = self.map[s.row + 1][s.col];
        }

        let valid_left = ['F', '-', 'L'];
        let valid_top = ['7', '|', 'F'];
        let valid_right = ['7', '-', 'J'];
        let valid_bottom = ['J', '|', 'L'];

        let valid = (
            valid_left.contains(&left),
            valid_top.contains(&top),
            valid_right.contains(&right),
            valid_bottom.contains(&bottom),
        );

        match valid {
            (true, true, _, _) => self.map[s.row][s.col] = 'J',
            (true, _, true, _) => self.map[s.row][s.col] = '-',
            (true, _, _, true) => self.map[s.row][s.col] = '7',
            (_, true, true, _) => self.map[s.row][s.col] = 'L',
            (_, true, _, true) => self.map[s.row][s.col] = '|',
            (_, _, true, true) => self.map[s.row][s.col] = 'F',
            _ => {
                panic!("Could not figure out S");
            }
        }
    }

    pub fn sum_enclosing_tiles(&mut self) -> u32 {
        let mut sum: u32 = 0;
        //let n_rows = self.map.len();
        //let n_cols = self.map[0].len();

        let mut flag: bool;

        // These chars denote whether we are passing into
        // or out of the enclosed pipe area as
        // we traverse from top-left to bottom-right
        let chars = ['F', '|', '7'];

        for row in &mut self.map {
            flag = false;
            for char in row {
                if chars.contains(char) {
                    flag ^= true; // toggle bool
                    continue;
                }
                if flag && *char == '~' {
                    sum += 1;
                    *char = 'I';
                }
            }
        }

        sum
    }

    pub fn print_map(&self) {
        for row in &self.map {
            for char in row {
                print!("{}", char);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
        let result = invoke(input);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_b() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let result = invoke(input);
        assert_eq!(result, "8");
    }

    #[test]
    fn test_c() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        let result = invoke(input);
        assert_eq!(result, "10");
    }
}
