use md5::{Digest, Md5};
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2015/04.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let input = input.trim();
    let mut hasher = Md5::new();
    for i in 0..9_999_999 {
        let secret = format!("{input}{i}");
        hasher.update(secret.as_bytes());
        let result = hasher.finalize_reset();
        let hash = format!("{result:x}");
        if hash.starts_with("000000") {
            println!("{secret} {hash}");
            return i.to_string();
        }
    }
    0.to_string()
}
