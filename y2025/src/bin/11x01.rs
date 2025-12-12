use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../../../puzzle_data/2025/11.txt").trim();
    let out = invoke(input);
    println!("{out}");
}

fn invoke(input: &str) -> String {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let u = tokens.next().expect("Should be a U node").replace(":", "");
        graph.entry(u.clone()).or_default();
        for token in tokens {
            graph
                .get_mut(&u)
                .expect("Should exist")
                .push(token.to_string());
        }
    }

    let mut total = 0;
    let mut paths: VecDeque<String> = VecDeque::new();
    paths.push_back("you".to_string());
    let out = "out".to_string();

    while let Some(path) = paths.pop_front() {
        let next = graph.get(&path).expect("Should be in the graph");
        for n in next {
            if *n == out {
                total += 1;
            } else {
                paths.push_back(n.clone());
            }
        }
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let result = invoke(input);
        assert_eq!(result, "5");
    }
}
