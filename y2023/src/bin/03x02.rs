use regex::Regex;
use std::fs;
use std::{collections::HashMap, vec};
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/03.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(schematic: &str) -> String {
    let row_length = schematic.find("\n").unwrap() as i32;

    // dbg!(row_length);
    //
    let schematic = schematic.replace("\n", "");
    let star_re = Regex::new(r"\*").unwrap();
    let star_indices: Vec<i32> = star_re
        .find_iter(&schematic)
        .map(|f| f.start() as i32)
        .collect();
    let mut star_map: HashMap<i32, Vec<i32>> = HashMap::new();
    for si in star_indices {
        star_map.insert(si, vec![]);
    }

    // dbg!(&star_map);

    let numbers_re = Regex::new(r"\d+").unwrap();
    let numbers = numbers_re.find_iter(&schematic);
    let mut neighbours: Vec<i32> = vec![];
    let mut remainder: i32;
    for number in numbers {
        // println!("---");
        neighbours.clear();
        let num = number.as_str().parse::<i32>().unwrap();

        // dbg!(num);

        let start_idx = number.start() as i32;
        remainder = start_idx % row_length;
        if remainder > 0 {
            neighbours.push(start_idx - 1);
            neighbours.push(start_idx - row_length - 1);
            neighbours.push(start_idx + row_length - 1);
        }
        let end_idx = number.end() as i32 - 1;

        // dbg!(start_idx, end_idx);

        remainder = end_idx % row_length;
        if remainder != 0 {
            neighbours.push(end_idx + 1);
            neighbours.push(end_idx - row_length + 1);
            neighbours.push(end_idx + row_length + 1);
        }
        for idx in start_idx..end_idx + 1 {
            neighbours.push(idx - row_length);
            neighbours.push(idx + row_length);
        }

        // dbg!(&neighbours);

        for n in &neighbours {
            let star = star_map.get_mut(n);
            if star.is_some() {
                //	dbg!(num, n);
                let star = star.unwrap();
                star.push(num);
            }
        }
    }

    // Iterate through the map and add up the digits
    let mut sum = 0;
    for (_, v) in star_map {
        if v.len() == 2 {
            //dbg!(&v);
            sum += v[0] * v[1];
        }
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = invoke(input);
        assert_eq!(result, "4361");
    }
}
