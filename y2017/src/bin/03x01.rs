use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2017/03.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let x = input.trim().parse::<f64>().unwrap();
    let rn = (x.sqrt().ceil() / 2.0).floor();
    //println!("Ring: {}", rn);
    if rn == 0.0 {
        return "0".to_string();
    }

    let a = x - (2.0 * rn - 1.0).powi(2);
    let b = 4.0 * (2.0 * rn - 1.0) + 4.0;
    let offset = a % b;
    let side = (offset % (2.0 * rn) - rn).abs();

    let dist = rn + side;
    dist.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "1";
        let result = invoke(input);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_b() {
        let input = "12";
        let result = invoke(input);
        assert_eq!(result, "3");
    }

    #[test]
    fn test_c() {
        let input = "23";
        let result = invoke(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_d() {
        let input = "1024";
        let result = invoke(input);
        assert_eq!(result, "31");
    }
}
