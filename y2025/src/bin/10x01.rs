use itertools::Itertools;

fn main() {
    let input = include_str!("../../../puzzle_data/2025/10.txt").trim();
    let out = invoke(input);
    println!("{out}");
}

pub struct Machine {
    indicator: Vec<bool>,
    btns: Vec<Vec<usize>>,
}

impl Machine {
    fn minimum_on_sequence(&self) -> usize {
        let mut presses = 0;
        loop {
            presses += 1;
            let combinations = self.btns.iter().combinations(presses);
            for sequence in combinations {
                let mut lights = vec![false; self.indicator.len()];
                for press in sequence {
                    for indicator in press {
                        lights[*indicator] = !lights[*indicator];
                    }
                }
                // Check if the lights match
                let mut found = true;
                for (l1, l2) in self.indicator.iter().zip(lights) {
                    if *l1 != l2 {
                        found = false;
                        break;
                    }
                }
                if found {
                    return presses;
                }
            }
        }
    }
}

fn invoke(input: &str) -> String {
    // TODO: Pass the results
    let mut total = 0;
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let indicator: Vec<bool> = tokens
            .next()
            .expect("Should be some indicator lights")
            .replace("[", "")
            .replace("]", "")
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!("Should not get here"),
            })
            .collect();
        let mut btns: Vec<Vec<usize>> = Vec::new();
        for token in tokens {
            if token.contains("(") {
                let btn: Vec<usize> = token
                    .replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect();
                btns.push(btn);
            }
        }
        let machine = Machine { indicator, btns };
        let presses = machine.minimum_on_sequence();
        println!("Minimum Presses Required: {}", presses);
        total += presses;
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

        let result = invoke(input);
        assert_eq!(result, "7");
    }
}
