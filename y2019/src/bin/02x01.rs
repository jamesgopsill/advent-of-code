use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2019/02.txt");
    let out = invoke(input);
    println!("{out}");
    bench(invoke, input);
}

fn invoke(input: &str) -> String {
    let mut items: Vec<usize> = input
        .split(",")
        .map(|v| v.trim().parse::<usize>().unwrap())
        .collect();
    items[1] = 12;
    items[2] = 2;
    run(items)
}

fn run(mut seq: Vec<usize>) -> String {
    for i in (0..seq.len()).step_by(4) {
        let intcode = seq[i];
        match intcode {
            1 => {
                let idx_one = seq[i + 1];
                let idx_two = seq[i + 2];
                let register = seq[i + 3];
                seq[register] = seq[idx_one] + seq[idx_two];
            }
            2 => {
                let idx_one = seq[i + 1];
                let idx_two = seq[i + 2];
                let register = seq[i + 3];
                seq[register] = seq[idx_one] * seq[idx_two];
            }
            99 => break,
            _ => {
                panic!("Should not get here")
            }
        }
    }
    //println!("Revised: {seq:?}");
    seq[0].to_string()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_a() {
        let input = "1,0,0,0,99";
        let items: Vec<usize> = input
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let result = run(items);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_b() {
        let input = "2,3,0,3,99";
        let items: Vec<usize> = input
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let result = run(items);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_c() {
        let input = "2,4,4,5,99,0";
        let items: Vec<usize> = input
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let result = run(items);
        assert_eq!(result, "2");
    }

    #[test]
    fn test_d() {
        let input = "1,1,1,4,99,5,6,0,99";
        let items: Vec<usize> = input
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        let result = run(items);
        assert_eq!(result, "30");
    }
}
