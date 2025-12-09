fn main() {
    let input = include_str!("../../../puzzle_data/2025/07.txt").trim();
    let out = invoke(input);
    println!("{out}");
}

fn invoke(input: &str) -> String {
    let mut map: Vec<Vec<char>> = Vec::new();
    let lines = input.lines();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        map.push(chars);
    }

    let mut nmap: Vec<Vec<i64>> = Vec::new();
    for row in map.iter() {
        let mut nrow: Vec<i64> = Vec::new();
        for c in row.iter() {
            match c {
                'S' => nrow.push(1),
                '.' => nrow.push(0),
                '^' => nrow.push(-1),
                _ => panic!("unknown char."),
            }
        }
        nmap.push(nrow);
    }

    // Propagate the tachyon beam.
    for i in 0..nmap.len() {
        let (left, right) = nmap.split_at_mut(i + 1);
        let left = left.last_mut().unwrap();
        let Some(right) = right.first_mut() else {
            break;
        };
        let col_length = right.len();
        for (j, c) in left.iter().enumerate() {
            if *c > 0 {
                // println!("Tachyon Detected");
                match right[j] {
                    -1 => {
                        if j == 0 {
                            right[j + 1] += *c;
                            continue;
                        }
                        if j == col_length {
                            right[j - 1] += *c;
                            continue;
                        }
                        right[j - 1] += *c;
                        right[j + 1] += *c;
                    }
                    _ => {
                        right[j] += *c;
                    }
                }
            }
        }
    }

    /*
    for line in nmap.iter() {
        for v in line.iter() {
            if *v == 0 {
                print!("---|")
            } else {
                print!("{v:03}|")
            }
        }
        print!("\n");
    }*/

    let many_worlds: i64 = nmap.last().unwrap().iter().sum();
    many_worlds.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let result = invoke(input);
        assert_eq!(result, "40");
    }
}
