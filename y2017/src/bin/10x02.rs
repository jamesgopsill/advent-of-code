use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle_data/2017/10.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
}

fn invoke(input: &str) -> String {
    let mut sparse: [u8; 256] = [0u8; 256];
    for i in 0..=255u8 {
        sparse[i as usize] = i;
    }

    // Treat the input as bytes.
    let mut lengths: Vec<usize> = Vec::new();
    //println!("{}", input.is_ascii());
    for len in input.trim().as_bytes() {
        lengths.push(*len as usize);
    }

    // Add the suffix
    lengths.push(17);
    lengths.push(31);
    lengths.push(73);
    lengths.push(47);
    lengths.push(23);

    //println!("{:?}", sparse);
    //println!("{:?}", lengths);

    // 64 rounds.
    let hash_len = sparse.len();
    let mut start: usize = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for len in lengths.iter() {
            //println!("{:?}", hash);
            let end = start + len - 1;
            let half_len = len / 2;
            // println!("Start: {}, End: {}: Half: {}", start, end, half_len);
            for i in 0..half_len {
                let si = bounded_idx(start + i, hash_len);
                let ei = bounded_idx(end - i, hash_len);
                sparse.swap(si, ei);
            }
            start = bounded_idx(start + len + skip, hash_len);
            skip += 1;
        }
    }

    //println!("{:?}", sparse);

    // dense hash
    let mut dense: [u8; 16] = [0u8; 16];
    //println!("{:?}", dense);
    for i in 0..16 {
        let start = i * 16;
        let xor = xor(&sparse[start..][..16]);
        dense[i] = xor;
    }
    //println!("{:?}", dense);

    // Hexadecimal
    let mut out = format!("{dense:02X?}");
    //println!("{}", out);
    out = out.replace(" ", "");
    out = out.replace(",", "");
    out = out.replace("[", "");
    out = out.replace("]", "");
    out.to_lowercase()
}

fn bounded_idx(val: usize, vlen: usize) -> usize {
    if val == 0 || val == vlen - 1 {
        return val;
    }
    (val) % vlen
}

fn xor(slice: &[u8]) -> u8 {
    let mut xor = 0;
    for val in slice {
        xor ^= val;
    }
    xor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "";
        let result = invoke(input);
        assert_eq!(result, "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn test_b() {
        let input = "1,2,3";
        let result = invoke(input);
        assert_eq!(result, "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn test_xor() {
        let input: Vec<u8> = vec![65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
        let result = xor(&input);
        assert_eq!(result, 64);
    }
}
