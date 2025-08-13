#![allow(dead_code)]
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2024/15.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut map = CharMat2::new();
    let mut instructions: Vec<char> = vec![];
    for line in input.lines() {
        if line.starts_with("#") {
            let row: Vec<char> = line.chars().collect();
            map.add_row(row);
            continue;
        }
        if line.is_empty() {
            let row: Vec<char> = line.chars().collect();
            instructions.extend(row);
        }
    }
    // find robot
    let mut robot: [usize; 2] = [0, 0];
    for (y, row) in map.0.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '@' {
                robot = [x, y]
            }
        }
    }

    println!("Robot Starting Location: {:?}", robot);
    let x_max = map.0[0].len();
    let y_max = map.0.len();

    for i in instructions {
        //println!("-- Iteration {} --", i);
        //map.print();
        let [rx, ry] = robot;
        //println!("{} {}", rx, ry);
        match i {
            '^' => {
                let col = map.get_col_ref(rx);
                let mut replace: Vec<(usize, usize, char)> = vec![(rx, ry, '.')];
                for y in (1..=ry).rev() {
                    replace.push((rx, y - 1, *col[y]));
                    match col[y] {
                        '.' => {
                            replace.pop();
                            //println!("{:?}", replace);
                            for (nx, ny, c) in replace {
                                map.0[ny][nx] = c;
                            }
                            robot[1] -= 1;
                            break;
                        }
                        '#' => break,
                        _ => {}
                    }
                }
            }
            'v' => {
                let col = map.get_col_ref(rx);
                // println!("{:?}", col);
                let mut replace: Vec<(usize, usize, char)> = vec![(rx, ry, '.')];
                for y in ry..y_max {
                    replace.push((rx, y + 1, *col[y]));
                    match col[y] {
                        '.' => {
                            replace.pop();
                            //println!("{:?}", replace);
                            for (nx, ny, c) in replace {
                                map.0[ny][nx] = c;
                            }
                            robot[1] += 1;
                            break;
                        }
                        '#' => break,
                        _ => {}
                    }
                }
            }
            '>' => {
                let row = map.get_row_ref(ry);
                // println!("{:?}", row);
                let mut replace: Vec<(usize, usize, char)> = vec![(rx, ry, '.')];
                for x in rx..x_max {
                    replace.push((x + 1, ry, *row[x]));
                    match row[x] {
                        '.' => {
                            replace.pop();
                            //println!("{:?}", replace);
                            for (nx, ny, c) in replace {
                                map.0[ny][nx] = c;
                            }
                            robot[0] += 1;
                            break;
                        }
                        '#' => break,
                        _ => {}
                    }
                }
            }
            '<' => {
                let row = map.get_row_ref(ry);
                let mut replace: Vec<(usize, usize, char)> = vec![(rx, ry, '.')];
                for x in (1..=rx).rev() {
                    replace.push((x - 1, ry, *row[x]));
                    match row[x] {
                        '.' => {
                            replace.pop();
                            //println!("{:?}", replace);
                            for (nx, ny, c) in replace {
                                map.0[ny][nx] = c;
                            }
                            robot[0] -= 1;
                            break;
                        }
                        '#' => break,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    map.print();

    let mut ans: u32 = 0;
    for (y, row) in map.0.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'O' {
                ans += (100 * y as u32) + x as u32
            }
        }
    }

    ans.to_string()
}

struct CharMat2(Vec<Vec<char>>);

impl CharMat2 {
    fn new() -> Self {
        Self(vec![])
    }

    fn add_row(&mut self, row: Vec<char>) {
        self.0.push(row);
    }

    fn get_ref(&self, x: usize, y: usize) -> &char {
        &self.0[y][x]
    }

    fn get_mut_ref(&mut self, x: usize, y: usize) -> &mut char {
        &mut self.0[y][x]
    }

    fn get_row_ref(&self, y: usize) -> Vec<&char> {
        let mut v = vec![];
        for c in self.0[y].iter() {
            v.push(c);
        }
        v
    }

    fn get_mut_row_ref(&mut self, y: usize) -> Vec<&mut char> {
        let mut v: Vec<&mut char> = vec![];
        for c in self.0[y].iter_mut() {
            v.push(c);
        }
        v
    }

    fn get_col_ref(&self, x: usize) -> Vec<&char> {
        let mut v = vec![];
        for c in self.0.iter() {
            v.push(&c[x]);
        }
        v
    }

    fn get_col_mut_ref(&mut self, x: usize) -> Vec<&mut char> {
        let mut v = vec![];
        for c in self.0.iter_mut() {
            v.push(&mut c[x]);
        }
        v
    }

    fn print(&self) {
        for row in self.0.iter() {
            for c in row {
                print!("{}", c);
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let result = invoke(input);
        assert_eq!(result, "2028");
    }

    #[test]
    fn test_b() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let result = invoke(input);
        assert_eq!(result, "10092");
    }
}
