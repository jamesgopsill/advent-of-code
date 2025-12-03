use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2025/02.txt").trim();
    let out = invoke(input);
    println!("{out}");
    bench(invoke, input);
}

fn invoke(input: &str) -> String {
    // Parse the ranges from the string
    let ranges = input.split(',');
    // Create a vec to store our invalid ids
    let mut invalid_ids: Vec<u64> = Vec::new();
    for range in ranges {
        // Get the min, max in the range
        let (min, max) = range.split_once('-').unwrap();
        let min: u64 = min.parse().unwrap();
        let max: u64 = max.parse().unwrap();
        for id in min..=max {
            // Turn it into a string
            let s = id.to_string();
            // Ignore if an odd length as we cannot repeat.
            if s.len() % 2 != 0 {
                continue;
            }
            // Split in the middle and check if they are equal
            let (left, right) = s.split_at(s.len() / 2);
            if left == right {
                invalid_ids.push(id);
            }
        }
    }
    let sum: u64 = invalid_ids.iter().sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = invoke(input);
        assert_eq!(result, "1227775554");
    }
}
