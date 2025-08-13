use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle_data/2017/10.txt").unwrap();
    let out = invoke(&input, 256);
    println!("{}", out);
}

fn invoke(input: &str, len: usize) -> String {
    let mut hash: Vec<usize> = Vec::new();
    for i in 0..len {
        hash.push(i);
    }

    let mut lengths: Vec<usize> = Vec::new();
    for len in input.split(",") {
        let len = len.trim().parse::<usize>().unwrap();
        lengths.push(len);
    }

    let hash_len = hash.len();
    let mut start: usize = 0;
    let mut end: usize;
    let mut half_len: usize;
    for (skip, len) in lengths.iter().enumerate() {
        //println!("{:?}", hash);
        end = start + len - 1;
        half_len = len / 2;
        // println!("Start: {}, End: {}: Half: {}", start, end, half_len);
        for i in 0..half_len {
            let si = bounded_idx(start + i, hash_len);
            let ei = bounded_idx(end - i, hash_len);
            hash.swap(si, ei);
        }
        start = bounded_idx(start + len + skip, hash_len);
    }
    (hash[0] * hash[1]).to_string()
}

fn bounded_idx(val: usize, vlen: usize) -> usize {
    if val == 0 || val == vlen - 1 {
        return val;
    }
    (val) % vlen
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "3, 4, 1, 5";
        let result = invoke(input, 5);
        assert_eq!(result, "12");
    }
}
