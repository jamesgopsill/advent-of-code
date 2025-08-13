use std::{collections::HashSet, mem::swap};

use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2024/10.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut map: Vec<Vec<u32>> = vec![];
    for lines in input.lines() {
        let mut row: Vec<u32> = vec![];
        for c in lines.chars() {
            match c {
                '.' => {
                    row.push(55);
                }
                _ => {
                    let c = c.to_digit(10).unwrap();
                    row.push(c);
                }
            }
        }
        map.push(row);
    }

    let mut trails: Vec<Trail> = vec![];
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 0 {
                let t = Trail::new([i, j], &map);
                trails.push(t);
            }
        }
    }

    let mut ans: u32 = 0;
    for trail in trails.iter_mut() {
        //println!("{:?}", trail.start);
        trail.walk();
        //println!("{}", trail.ends);
        ans += trail.ends;
        // break;
    }

    ans.to_string()
}

struct Trail<'a> {
    map: &'a Vec<Vec<u32>>,
    start: [usize; 2],
    ends: u32,
}

impl<'a> Trail<'a> {
    fn new(start: [usize; 2], map: &'a Vec<Vec<u32>>) -> Self {
        Self {
            start,
            map,
            ends: 0,
        }
    }

    fn walk(&mut self) {
        // N.b. originally tried vecs with dedup but dedup missed some items.
        let row_max = self.map.len() - 1;
        let col_max = self.map[0].len() - 1;
        let mut current_locs: HashSet<[usize; 2]> = HashSet::new();
        current_locs.insert(self.start);
        let mut next_locs: HashSet<[usize; 2]> = HashSet::new();
        for i in 1..=9 {
            for loc in current_locs.iter() {
                // Top
                if loc[0] > 0 && self.map[loc[0] - 1][loc[1]] == i {
                    next_locs.insert([loc[0] - 1, loc[1]]);
                }
                // Bottom
                if loc[0] < row_max && self.map[loc[0] + 1][loc[1]] == i {
                    next_locs.insert([loc[0] + 1, loc[1]]);
                }
                // Left
                if loc[1] > 0 && self.map[loc[0]][loc[1] - 1] == i {
                    next_locs.insert([loc[0], loc[1] - 1]);
                }
                // Right
                if loc[1] < col_max && self.map[loc[0]][loc[1] + 1] == i {
                    next_locs.insert([loc[0], loc[1] + 1]);
                }
            }
            //println!("{i} {:?}", next_locs);
            swap(&mut current_locs, &mut next_locs);
            next_locs.clear();
        }
        // unique checks
        //println!("Final: {:?}", current_locs);
        self.ends = current_locs.len() as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
";
        let result = invoke(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_b() {
        let input = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";
        let result = invoke(input);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_c() {
        let input = "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
";
        let result = invoke(input);
        assert_eq!(result, "3");
    }

    #[test]
    fn test_d() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        let result = invoke(input);
        assert_eq!(result, "36");
    }
}
