#![allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    fs,
};
use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2019/03.txt");
    let out = invoke(input);
    println!("{out}");
    //bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let mut grid: HashMap<(i64, i64), [u64; 2]> = HashMap::new();

    let wires = input.lines();
    for (wid, wire) in wires.enumerate() {
        println!("Wire: {wid}");
        let path = wire.split(",");
        // Ignore indicating the first value as this is
        // where they start.
        let mut loc = (0, 0);
        let mut dist = 0;
        for step in path {
            let mut chars = step.chars();
            let dir: char = chars.next().unwrap();
            let val: u64 = chars.as_str().parse().unwrap();
            match dir {
                'U' => {
                    for i in (1..=val) {
                        dist += 1;
                        loc.1 += 1;
                        if let Some(cell) = grid.get_mut(&loc) {
                            if cell[wid] > dist {
                                cell[wid] = dist;
                            };
                        } else {
                            let mut cell = [u64::MAX, u64::MAX];
                            cell[wid] = dist;
                            grid.insert(loc, cell);
                        }
                    }
                }
                'D' => {
                    for i in (1..=val) {
                        dist += 1;
                        loc.1 -= 1;
                        if let Some(cell) = grid.get_mut(&loc) {
                            if cell[wid] > dist {
                                cell[wid] = dist;
                            };
                        } else {
                            let mut cell = [u64::MAX, u64::MAX];
                            cell[wid] = dist;
                            grid.insert(loc, cell);
                        }
                    }
                }
                'R' => {
                    for i in (1..=val) {
                        dist += 1;
                        loc.0 += 1;
                        if let Some(cell) = grid.get_mut(&loc) {
                            if cell[wid] > dist {
                                cell[wid] = dist;
                            };
                        } else {
                            let mut cell = [u64::MAX, u64::MAX];
                            cell[wid] = dist;
                            grid.insert(loc, cell);
                        }
                    }
                }
                'L' => {
                    for i in (1..=val) {
                        dist += 1;
                        loc.0 -= 1;
                        if let Some(cell) = grid.get_mut(&loc) {
                            if cell[wid] > dist {
                                cell[wid] = dist;
                            };
                        } else {
                            let mut cell = [u64::MAX, u64::MAX];
                            cell[wid] = dist;
                            grid.insert(loc, cell);
                        }
                    }
                }
                _ => {
                    panic!("Should not get here.")
                }
            }
        }
    }

    // Find the overlaps and compute the manhatten distance.
    let mut lowest = u64::MAX;
    for (loc, cell) in grid.iter() {
        //println!("{loc:?}: {cell:?}");
        if cell[0] < u64::MAX && cell[1] < u64::MAX {
            let time = cell.iter().sum();
            if time < lowest {
                lowest = time;
            }
        }
    }

    lowest.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let result = invoke(input);
        assert_eq!(result, "30");
    }

    #[test]
    fn test_b() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let result = invoke(input);
        assert_eq!(result, "610");
    }

    #[test]
    fn test_c() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let result = invoke(input);
        assert_eq!(result, "410");
    }
}
