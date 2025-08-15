use std::collections::HashMap;
//use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2019/03.txt");
    let out = invoke(input);
    println!("{out}");
    //bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let mut grid: HashMap<(i64, i64), [bool; 2]> = HashMap::new();

    let mut update_grid = |loc: (i64, i64), id: usize| {
        if let Some(cell) = grid.get_mut(&loc) {
            cell[id] = true;
        } else {
            let mut cell = [false, false];
            cell[id] = true;
            grid.insert(loc, cell);
        }
    };

    let wires = input.lines();
    for (wid, wire) in wires.enumerate() {
        let path = wire.split(",");
        // Ignore indicating the first value as this is
        // where they start.
        let mut loc = (0, 0);
        for step in path {
            let mut chars = step.chars();
            let dir: char = chars.next().unwrap();
            let val: u64 = chars.as_str().parse().unwrap();
            match dir {
                'U' => {
                    for _i in 1..=val {
                        loc.1 += 1;
                        update_grid(loc, wid);
                    }
                }
                'D' => {
                    for _i in 1..=val {
                        loc.1 -= 1;
                        update_grid(loc, wid);
                    }
                }
                'R' => {
                    for _i in 1..=val {
                        loc.0 += 1;
                        update_grid(loc, wid);
                    }
                }
                'L' => {
                    for _i in 1..=val {
                        loc.0 -= 1;
                        update_grid(loc, wid);
                    }
                }
                _ => {
                    panic!("Should not get here.")
                }
            }
        }
    }

    // Find the overlaps and compute the manhatten distance.
    let mut closest = u64::MAX;
    for (loc, cell) in grid.iter() {
        if cell[0] && cell[1] {
            let manhatten = (loc.0.abs() + loc.1.abs()) as u64;
            if manhatten < closest {
                closest = manhatten;
            }
        }
    }

    closest.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let result = invoke(input);
        assert_eq!(result, "6");
    }

    #[test]
    fn test_b() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let result = invoke(input);
        assert_eq!(result, "159");
    }

    #[test]
    fn test_c() {
        let input =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let result = invoke(input);
        assert_eq!(result, "135");
    }
}
