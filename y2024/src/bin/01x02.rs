use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2024/01.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut a: Vec<u32> = vec![];
    let mut b: Vec<u32> = vec![];

    for line in input.lines() {
        let mut elements = line.split_whitespace();
        a.push(elements.next().unwrap().parse::<u32>().unwrap());
        b.push(elements.next().unwrap().parse::<u32>().unwrap());
    }

    let mut similarity: u32 = 0;
    for x in a.iter() {
        let mut count: u32 = 0;
        for y in b.iter() {
            if x == y {
                count += 1;
            }
        }
        similarity += x * count;
    }
    similarity.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let result = invoke(
            "3   4
4   3
2   5
1   3
3   9
3   3
",
        );
        assert_eq!(result, "31");
    }
}
