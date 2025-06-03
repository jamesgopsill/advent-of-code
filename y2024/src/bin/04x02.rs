use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/01.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut map: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let mut row: Vec<char> = vec![];
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }
    let mut count: u32 = 0;
    let row_count = map.len();
    let col_count = map[0].len();
    for i in 1..row_count - 1 {
        for j in 1..col_count - 1 {
            if map[i][j] != 'A' {
                continue;
            }

            // M M
            //  A
            // S S
            if map[i - 1][j - 1] == 'M'
                && map[i + 1][j - 1] == 'M'
                && map[i - 1][j + 1] == 'S'
                && map[i + 1][j + 1] == 'S'
            {
                count += 1;
            }
            // M S
            //  A
            // M S
            if map[i - 1][j - 1] == 'M'
                && map[i + 1][j - 1] == 'S'
                && map[i - 1][j + 1] == 'M'
                && map[i + 1][j + 1] == 'S'
            {
                count += 1;
            }
            // S S
            //  A
            // M M
            if map[i - 1][j - 1] == 'S'
                && map[i + 1][j - 1] == 'S'
                && map[i - 1][j + 1] == 'M'
                && map[i + 1][j + 1] == 'M'
            {
                count += 1;
            }
            // S M
            //  A
            // S M
            if map[i - 1][j - 1] == 'S'
                && map[i + 1][j - 1] == 'M'
                && map[i - 1][j + 1] == 'S'
                && map[i + 1][j + 1] == 'M'
            {
                count += 1;
            }
        }
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let result = invoke(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
",
        );
        assert_eq!(result, "9");
    }
}
