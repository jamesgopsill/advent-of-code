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

    // Propagate the tachyon beam.
    let mut splitters = 0;
    for i in 0..map.len() {
        let (left, right) = map.split_at_mut(i + 1);
        let left = left.last_mut().unwrap();
        let Some(right) = right.first_mut() else {
            break;
        };
        for (j, c) in left.iter().enumerate() {
            if *c == '|' || *c == 'S' {
                // println!("Tachyon Detected");
                match right[j] {
                    '^' => {
                        splitters += 1;
                        if j == 0 {
                            right[j + 1] = '|';
                            continue;
                        }
                        if j == right.len() {
                            right[j - 1] = '|';
                            continue;
                        }
                        right[j - 1] = '|';
                        right[j + 1] = '|';
                    }
                    '.' => right[j] = '|',
                    _ => {}
                }
            }
        }
    }

    /*
    for line in map.iter() {
        let s: String = line.iter().collect();
        println!("{s}");
    }*/

    splitters.to_string()
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
        assert_eq!(result, "21");
    }
}
