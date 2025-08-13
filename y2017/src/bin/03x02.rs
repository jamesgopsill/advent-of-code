use std::{collections::HashMap, fs};
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2017/03.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let inp = input.trim().parse::<u64>().unwrap();
    let mut map: HashMap<(i64, i64), u64> = HashMap::new();
    map.insert((0, 0), 1);

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut d = Direction::East;
    let mut sum: u64;

    loop {
        match d {
            Direction::North => y += 1,
            Direction::East => x += 1,
            Direction::South => y -= 1,
            Direction::West => x -= 1,
        }

        // sum values around
        sum = 0;
        for cx in x - 1..=x + 1 {
            for cy in y - 1..=y + 1 {
                if let Some(v) = map.get(&(cx, cy)) {
                    sum += *v;
                }
            }
        }
        if sum > inp {
            break;
        }
        map.insert((x, y), sum);

        // Determine the next direction;
        match d {
            Direction::North => {
                if map.contains_key(&(x - 1, y)) {
                    d = Direction::North;
                } else {
                    d = Direction::West;
                }
            }
            Direction::East => {
                if map.contains_key(&(x, y + 1)) {
                    d = Direction::East;
                } else {
                    d = Direction::North;
                }
            }
            Direction::South => {
                if map.contains_key(&(x + 1, y)) {
                    d = Direction::South;
                } else {
                    d = Direction::East;
                }
            }
            Direction::West => {
                if map.contains_key(&(x, y - 1)) {
                    d = Direction::West;
                } else {
                    d = Direction::South;
                }
            }
        }
    }

    sum.to_string()
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "5";
        let result = invoke(input);
        assert_eq!(result, "10");
    }
}
