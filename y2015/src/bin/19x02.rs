use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2015/19.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

// Too computationally intensive.
pub fn invoke(input: &str) -> String {
    let mut lines: Vec<&str> = input.lines().collect();
    let mut many_to_one: HashMap<&str, &str> = HashMap::new();
    // Create the list of changes
    for line in lines.iter() {
        if !line.contains("=>") {
            break;
        }
        let (a, b) = line.split_once("=>").unwrap();
        let a = a.trim();
        let b = b.trim();
        many_to_one.insert(b, a);
    }

    let medicine_molecule = lines.pop().unwrap().to_string();
    let mut molecules_a: HashSet<String> = HashSet::new();
    molecules_a.insert(medicine_molecule);
    let mut molecules_b: HashSet<String> = HashSet::new();
    let mut n = 0;
    // While we still have molecules to back track
    while !molecules_a.is_empty() {
        n += 1;
        // For each molecule string
        for molecule in molecules_a.iter() {
            // Review each pair we could translate
            for (a, b) in many_to_one.iter() {
                let idxs: Vec<usize> = molecule.match_indices(a).map(|(i, _)| i).collect();
                for idx in idxs {
                    let mut m = molecule.clone();
                    m.replace_range(idx..idx + a.len(), b);
                    molecules_b.insert(m);
                }
            }
        }
        println!("{}", molecules_b.len());
        // println!("{:?}", molecules_b);
        // Check if any have returned to e
        let mut flag = false;
        for molecule in molecules_b.iter() {
            if molecule == "e" {
                flag = true;
                break;
            }
        }
        if flag {
            break;
        }
        swap(&mut molecules_a, &mut molecules_b);
        molecules_b.clear();
    }
    n.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let result = invoke(
            "e => H
e => O
H => HO
H => OH
O => HH

HOH
",
        );
        assert_eq!(result, "3");
    }
}
