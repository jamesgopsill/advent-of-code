use std::{collections::HashMap, vec};

use regex::Regex;

pub fn part_02(schematic: String) -> i32 {
    let row_length = schematic.find("\n").unwrap() as i32;
    dbg!(row_length);
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

    //dbg!(&star_map);

    let numbers_re = Regex::new(r"\d+").unwrap();
    let numbers = numbers_re.find_iter(&schematic);
    let mut neighbours: Vec<i32> = vec![];
    let mut remainder: i32;
    for number in numbers {
        println!("---");
        neighbours.clear();
        let num = number.as_str().parse::<i32>().unwrap();
        println!("Value: {}", num);
        let start_idx = number.start() as i32;
        remainder = start_idx % row_length;
        if remainder > 0 {
            neighbours.push(start_idx - 1);
            neighbours.push(start_idx - row_length - 1);
            neighbours.push(start_idx + row_length - 1);
        }
        let end_idx = number.end() as i32 - 1;
        println!("{} {}", start_idx, end_idx);
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
        println!("Neighbour Indices: {:?}", neighbours);
        for n in &neighbours {
            let star = star_map.get_mut(n);
            if star.is_some() {
                println!("{} next to star {}", num, n);
                let star = star.unwrap();
                star.push(num.clone());
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

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_02() {
        let input = fs::read_to_string("src/a03/input_01.txt")
            .expect("Should have been able to read the file");
        let result = part_02(input);
        assert_eq!(result, 467835);
    }
}
