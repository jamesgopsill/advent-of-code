use std::collections::HashMap;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2024/12.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
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

    //println!("{:?}", plots);

    // Now to do the calculation
    let max_idx = (rows_max * cols_max) as u32;
    let mut total_price: u32 = 0;
    for p in 0..=max_idx {
        //println!("Idx: {}", p);
        let mut count: u32 = 0;
        let mut horizontals: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut verticals: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut diags: u32 = 0;
        for i in 0..rows_max {
            for j in 0..cols_max {
                if plots[i][j] != p {
                    continue;
                }
                count += 1;
                // top
                if i == 0 {
                    if horizontals.contains_key(&i) {
                        horizontals.get_mut(&i).unwrap().push(j);
                    } else {
                        horizontals.insert(i, vec![j]);
                    }
                } else if garden[i - 1][j] != garden[i][j] {
                    if horizontals.contains_key(&i) {
                        horizontals.get_mut(&i).unwrap().push(j);
                    } else {
                        horizontals.insert(i, vec![j]);
                    }
                }
                // bottom
                let below = i + 1;
                if i == rows_max - 1 {
                    if horizontals.contains_key(&below) {
                        horizontals.get_mut(&below).unwrap().push(j);
                    } else {
                        horizontals.insert(below, vec![j]);
                    }
                } else if garden[below][j] != garden[i][j] {
                    if horizontals.contains_key(&below) {
                        horizontals.get_mut(&below).unwrap().push(j);
                    } else {
                        horizontals.insert(below, vec![j]);
                    }
                }
                // left
                if j == 0 {
                    if verticals.contains_key(&j) {
                        verticals.get_mut(&j).unwrap().push(i);
                    } else {
                        verticals.insert(j, vec![i]);
                    }
                } else if garden[i][j - 1] != garden[i][j] {
                    if verticals.contains_key(&j) {
                        verticals.get_mut(&j).unwrap().push(i);
                    } else {
                        verticals.insert(j, vec![i]);
                    }
                }
                // right
                let right = j + 1;
                if j == cols_max - 1 {
                    if verticals.contains_key(&right) {
                        verticals.get_mut(&right).unwrap().push(i);
                    } else {
                        verticals.insert(right, vec![i]);
                    }
                } else if garden[i][right] != garden[i][j] {
                    if verticals.contains_key(&right) {
                        verticals.get_mut(&right).unwrap().push(i);
                    } else {
                        verticals.insert(right, vec![i]);
                    }
                }

                // diag check (got to be different plots)
                if i > 0 && i < rows_max - 1 && j > 0 && j < rows_max - 1 {
                    // AB
                    // BA
                    if plots[i][j] == plots[i + 1][j + 1]
                        && plots[i][j] != plots[i + 1][j]
                        && plots[i][j] != plots[i][j + 1]
                    {
                        /*
                        println!(
                            "*{}*{}\n{}{}",
                            plots[i][j],
                            plots[i][j + 1],
                            plots[i + 1][j],
                            plots[i + 1][j + 1],
                        );
                        println!("Diag 1: {} {}", i, j);
                        diags += 1;
                        */
                    }
                    // BA
                    // AB
                    if plots[i][j] == plots[i - 1][j + 1]
                        && plots[i][j] != plots[i - 1][j]
                        && plots[i][j] != plots[i][j + 1]
                    {
                        /*
                        println!(
                            "{}{}\n*{}*{}",
                            plots[i - 1][j],
                            plots[i - 1][j + 1],
                            plots[i][j],
                            plots[i][j + 1],
                        );
                        println!("Diag 2: {} {}", i, j);
                        */
                        diags += 1;
                    }
                }
            }
        }

        // calculate the sides
        let mut sides: u32 = 0;
        //println!("Diags: {}", diags);
        sides += diags * 2;
        //println!("Horizontals");
        for (_k, mut h) in horizontals {
            h.sort();
            //println!("{}: {:?}", k, h);
            sides += 1;
            if h.len() == 1 {
                continue;
            }
            h.sort();
            for win in h.windows(2) {
                if win[0] + 1 != win[1] {
                    sides += 1
                }
            }
        }
        //println!("Verticals");
        for (_k, mut v) in verticals {
            v.sort();
            //println!("{}: {:?}", k, v);
            sides += 1;
            if v.len() == 1 {
                continue;
            }
            for win in v.windows(2) {
                if win[0] + 1 != win[1] {
                    sides += 1
                }
            }
        }
        //println!("Sides: {}", sides);
        //println!("---");
        total_price += count * sides;
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
        assert_eq!(result, "80");
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
        assert_eq!(result, "436");
    }

    #[test]
    fn test_c() {
        let result = invoke(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
",
        );
        assert_eq!(result, "236");
    }

    // TODO: solve the crossing over.
    #[test]
    fn test_d() {
        let result = invoke(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
",
        );
        assert_eq!(result, "368");
    }
}
