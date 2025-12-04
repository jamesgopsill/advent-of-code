fn main() {
    let input = include_str!("../../../puzzle_data/2025/04.txt").trim();
    let out = invoke(input);
    println!("{out}");
}

fn invoke(input: &str) -> String {
    let mut map: Vec<Vec<char>> = Vec::new();
    for row in input.lines() {
        let col: Vec<char> = row.chars().collect();
        map.push(col);
    }
    let mut copy = map.clone();
    let mut accessible = 0;
    let row_count = map.len() - 1;
    let col_count = map[0].len() - 1;
    for (i, r) in map.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c != '@' {
                continue;
            }
            let row_min = i.saturating_sub(1);
            let row_max = (i + 1).clamp(0, row_count);
            let col_min = j.saturating_sub(1);
            let col_max = (j + 1).clamp(0, col_count);
            // Note. start as minus one as we will include ourselves.
            let mut count = -1;
            for row in map[row_min..=row_max].iter() {
                for ch in row[col_min..=col_max].iter() {
                    if *ch == '@' {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                accessible += 1;
                copy[i][j] = 'x';
            }
        }
    }
    /*
    for row in copy {
        println!("{:?}", row);
    }
    */
    accessible.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let result = invoke(input);
        assert_eq!(result, "13");
    }
}
