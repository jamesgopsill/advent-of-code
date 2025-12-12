fn main() {
    let input = include_str!("../../../puzzle_data/2025/09.txt").trim();
    let out = invoke(input);
    println!("{out}");
}

pub struct Corner(u64, u64);

fn invoke(input: &str) -> String {
    let mut corners: Vec<Corner> = Vec::new();
    for line in input.lines() {
        let (x, y) = line.split_once(",").expect("Expected a comma.");
        let x: u64 = x.parse().expect("Expected a u64");
        let y: u64 = y.parse().expect("Expected a u64");
        let c = Corner(x, y);
        corners.push(c);
    }

    let mut largest = 0;
    for (i, c1) in corners.iter().enumerate() {
        for c2 in corners[i..].iter() {
            let x = c1.0.abs_diff(c2.0) + 1;
            let y = c1.1.abs_diff(c2.1) + 1;
            let area = x * y;
            if area > largest {
                largest = area
            }
        }
    }

    largest.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

        let result = invoke(input);
        assert_eq!(result, "50");
    }
}
