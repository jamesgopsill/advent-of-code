use std::collections::HashSet;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2015/03.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut santa = Santa::new();
    for c in input.chars() {
        santa.step(c);
    }
    santa.unique_locations.len().to_string()
}

struct Santa {
    current_location: [i32; 2],
    unique_locations: HashSet<[i32; 2]>,
}

impl Santa {
    fn new() -> Self {
        let current_location = [0, 0];
        let mut unique_locations = HashSet::new();
        unique_locations.insert(current_location);
        Self {
            current_location,
            unique_locations,
        }
    }

    fn step(
        &mut self,
        direction: char,
    ) {
        match direction {
            '^' => self.current_location[1] += 1,
            '>' => self.current_location[0] += 1,
            '<' => self.current_location[0] -= 1,
            'v' => self.current_location[1] -= 1,
            _ => {}
        }
        self.unique_locations.insert(self.current_location);
    }
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let result = invoke(">");
        assert_eq!(result, "2");
    }

    #[test]
    fn test_b() {
        let result = invoke("^>v<");
        assert_eq!(result, "4");
    }

    #[test]
    fn test_c() {
        let result = invoke("^v^v^v^v^v");
        assert_eq!(result, "2");
    }
}
