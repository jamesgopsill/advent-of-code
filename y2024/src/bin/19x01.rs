use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2024/19.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut lines = input.lines();
    // Get the stripes
    let mut stripes = lines
        .next()
        .unwrap()
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|f| f.trim())
        .collect::<Vec<&str>>();
    //stripes.sort_by(|a, b| b.len().cmp(&a.len()));
    stripes.sort_by_key(|a| a.len());
    println!("{:?}", stripes);

    // Filter the stripes for combinations of other stripes
    let mut filtered_stripes: Vec<&str> = vec![];
    for (i, s) in stripes.iter().enumerate() {
        if s.len() == 1 {
            filtered_stripes.push(s);
            continue;
        }
        let valid = depth_first_search(s, &stripes[(i + 1)..], 0);
        println!("{}", valid);
        if !valid {
            filtered_stripes.push(s);
        }
    }
    println!("{:?}", filtered_stripes);

    // Check which towels we can make.
    lines.next(); // skip the empty line in the input
    let mut valid_count = 0;
    for t in lines {
        println!("Towel: {}", t);
        let valid = depth_first_search(t, &filtered_stripes, 0);
        if valid {
            println!("Possible");
            valid_count += 1;
        } else {
            println!("Impossible");
        }
    }

    valid_count.to_string()
}

fn depth_first_search(
    towel: &str,
    stripes: &[&str],
    i: usize,
    //d: usize,
) -> bool {
    if i == towel.len() {
        return true;
    }
    for s in stripes.iter() {
        if towel[i..].starts_with(s) {
            let next = depth_first_search(towel, stripes, i + s.len());
            if next {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let result = invoke(input);
        assert_eq!(result, "6");
    }
}

/* too much memory searching together (recursion?)
let mut ptrs: Vec<usize> = vec![0];
loop {
    let mut valid = false;
    // Find all the potential next routes.
    let mut nptrs: Vec<usize> = vec![];
    for ptr in ptrs.iter() {
        for s in stripes.iter() {
            if t[*ptr..].starts_with(s) {
                nptrs.push(ptr + s.len());
            }
        }
    }
    println!("{}", nptrs.len());
    // If there aren't any the drop.
    if nptrs.len() == 0 {
        println!("{} impossible", t);
        break;
    }
    // Clear
    ptrs.clear();
    // Check if any reached the finish line.
    for nptr in nptrs.iter() {
        if *nptr == t.len() {
            println!("{} possible", t);
            valid = true;
            break;
        } else {
            ptrs.push(*nptr);
        }
    }
    if valid {
        println!("Break out");
        valid_count += 1;
        break;
    }
}

*/

/* Too simple (overlap and multiple solutions)
loop {
    let mut ni: usize = 0;
    for s in stripes.iter() {
        if t[i..].starts_with(s) {
            ni = i + s.len();
            break;
        }
    }
    if ni == t.len() {
        println!("{} possible", t);
        valid += 1;
        break;
    } else if ni > i {
        i = ni
    } else {
        println!("{} impossible", t);
        break;
    }
}
*/
