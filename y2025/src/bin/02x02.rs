// use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2025/02.txt").trim();
    let out = invoke(input);
    println!("{out}");
    //bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let ranges = input.split(',');
    let mut invalid_ids: Vec<u64> = Vec::new();
    for range in ranges {
        let (min, max) = range.split_once('-').unwrap();
        let min: u64 = min.parse().unwrap();
        let max: u64 = max.parse().unwrap();
        for id in min..max + 1 {
            let s = id.to_string();
            // Edge case. Lengths of one cannot repeat
            if s.len() == 1 {
                continue;
            }
            // So we're going to start with an empty pattern.
            // Increment it by one of the chars in the value.
            // Then we check if this pattern makes up the rest
            // of the string.
            let mut pat = String::new();
            for c in s.chars() {
                pat.push(c);
                if pat.len() > s.len().div_ceil(2) {
                    break;
                }
                let need = s.len().div_ceil(pat.len());
                let got = s.matches(&pat).count();
                if need == got {
                    invalid_ids.push(id);
                    break;
                }
            }
        }
    }
    // println!("{:?}", invalid_ids);
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
        assert_eq!(result, "4174379265");
    }
}
