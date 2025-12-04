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
    let mut removed: usize = 0;
    loop {
        let r = remove_rolls(&mut map);
        removed += r;
        if r == 0 {
            break;
        }
    }
    removed.to_string()
}

fn remove_rolls(m: &mut [Vec<char>]) -> usize {
    let row_count = m.len() - 1;
    let col_count = m[0].len() - 1;
    let mut to_remove: Vec<(usize, usize)> = Vec::new();
    for (i, r) in m.iter().enumerate() {
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
            for row in m[row_min..=row_max].iter() {
                for ch in row[col_min..=col_max].iter() {
                    if *ch == '@' {
                        count += 1;
                    }
                }
            }
            if count < 4 {
                to_remove.push((i, j));
            }
        }
    }
    for (i, j) in to_remove.iter() {
        m[*i][*j] = 'r';
    }
    to_remove.len()
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
        assert_eq!(result, "43");
    }
}
