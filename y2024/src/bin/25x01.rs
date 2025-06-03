use std::fs;
use std::iter::zip;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2024/25.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut keys: Vec<[i8; 5]> = vec![];
    let mut locks: Vec<[i8; 5]> = vec![];

    let patterns = input.split("\n\n");
    for pattern in patterns {
        let lines = pattern.split("\n").collect::<Vec<&str>>();
        let mut vals: [i8; 5] = [-1; 5];
        for row in lines.iter() {
            let chars = row.chars().collect::<Vec<char>>();
            for (i, c) in chars.iter().enumerate() {
                if *c == '#' {
                    vals[i] += 1
                }
            }
        }
        // Key or Lock
        match lines[0] {
            "#####" => locks.push(vals),
            _ => keys.push(vals),
        }
    }

    println!("Keys: {}", keys.len());
    println!("Locks: {}", locks.len());

    let mut fits: u32 = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let mut fit: bool = true;
            for (l, k) in zip(lock, key) {
                // println!("{} {}", l, k);
                if l + k > 5 {
                    fit = false;
                    break;
                }
            }
            if fit {
                println!("{:?} {:?} Fit", lock, key);
                fits += 1;
            } else {
                println!("{:?} {:?} Overlap", lock, key);
            }
        }
    }

    fits.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let result = invoke(input);
        assert_eq!(result, "3");
    }
}
