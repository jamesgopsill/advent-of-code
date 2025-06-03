use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2024/12.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut garden: Vec<Vec<char>> = vec![];
    let mut plots: Vec<Vec<u32>> = vec![];

    // Creating the initial map
    let mut n: u32 = 0;
    for line in input.lines() {
        let mut plot: Vec<char> = vec![];
        let mut plot_number: Vec<u32> = vec![];
        for c in line.chars() {
            plot.push(c);
            plot_number.push(n);
            n += 1;
        }
        garden.push(plot);
        plots.push(plot_number);
    }

    // Now looping to identify the plots
    let rows_max = garden.len();
    let cols_max = garden[0].len();
    let mut changes: u32;
    // loop for a long time
    for _ in 0..1_000_000 {
        changes = 0;
        for i in 0..rows_max {
            for j in 0..cols_max {
                // top
                if i > 0 && garden[i - 1][j] == garden[i][j] && plots[i - 1][j] < plots[i][j] {
                    plots[i][j] = plots[i - 1][j];
                    changes += 1;
                }
                // top
                if i < rows_max - 1
                    && garden[i + 1][j] == garden[i][j]
                    && plots[i + 1][j] < plots[i][j]
                {
                    plots[i][j] = plots[i + 1][j];
                    changes += 1;
                }
                // left
                if j > 0 && garden[i][j - 1] == garden[i][j] && plots[i][j - 1] < plots[i][j] {
                    plots[i][j] = plots[i][j - 1];
                    changes += 1;
                }
                // right
                if j < cols_max - 1
                    && garden[i][j + 1] == garden[i][j]
                    && plots[i][j + 1] < plots[i][j]
                {
                    plots[i][j] = plots[i][j + 1];
                    changes += 1;
                }
            }
        }
        if changes == 0 {
            // println!("Plots Clustered");
            break;
        }
    }

    // println!("{:?}", plots);

    // Now to do the calculation
    let max_idx = (rows_max * cols_max) as u32;
    let mut total_price: u32 = 0;
    for p in 0..=max_idx {
        let mut count: u32 = 0;
        let mut faces: u32 = 0;
        for i in 0..rows_max {
            for j in 0..cols_max {
                if plots[i][j] != p {
                    continue;
                }
                count += 1;
                let mut neighbours: u32 = 0;
                // top
                if i > 0 && garden[i - 1][j] == garden[i][j] {
                    neighbours += 1;
                }
                // top
                if i < rows_max - 1 && garden[i + 1][j] == garden[i][j] {
                    neighbours += 1;
                }
                // left
                if j > 0 && garden[i][j - 1] == garden[i][j] {
                    neighbours += 1;
                }
                // right
                if j < cols_max - 1 && garden[i][j + 1] == garden[i][j] {
                    neighbours += 1;
                }
                faces += 4 - neighbours;
            }
        }
        // println!("{} {} {}", count, faces, count * faces);
        total_price += count * faces;
    }
    total_price.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let result = invoke(
            "AAAA
BBCD
BBCC
EEEC
",
        );
        assert_eq!(result, "140");
    }

    #[test]
    fn test_b() {
        let result = invoke(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
",
        );
        assert_eq!(result, "772");
    }

    #[test]
    fn test_c() {
        let result = invoke(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
",
        );
        assert_eq!(result, "1930");
    }
}
