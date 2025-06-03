use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2024/09.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut disk: Vec<i32> = vec![];
    let mut file_id: i32 = 0;
    for (i, val) in input.trim().chars().enumerate() {
        match i % 2 {
            0 => {
                let block = val.to_digit(10).unwrap();
                for _ in 0..block {
                    disk.push(file_id);
                }
                file_id += 1;
            }
            _ => {
                let free = val.to_digit(10).unwrap();
                for _ in 0..free {
                    disk.push(-1);
                }
            }
        }
    }
    //println!("Before: {:?}", disk);
    //println!("Disk Size: {}", disk.len());
    for forwards in 0..disk.len() {
        //println!("{} {:?}", forwards, disk);
        let u = disk[forwards];
        if u < 0 {
            // Could be smarter here but playing it safe.
            let mut backwards = disk.len() - 1;
            loop {
                let v = disk[backwards];
                if v > -1 {
                    disk[forwards] = v;
                    disk[backwards] = -1;
                    break;
                }
                backwards -= 1;
                if forwards >= backwards {
                    break;
                }
            }
            if forwards >= backwards {
                break;
            }
        }
    }
    // println!("After: {:?}", disk);

    // checksum
    let mut checksum: u64 = 0;
    for (i, val) in disk.iter().enumerate() {
        // Reached the free space region at the end of the disk
        if *val < 0 {
            break;
        }
        checksum += i as u64 * *val as u64;
    }
    checksum.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "12345";
        let result = invoke(input);
        assert_eq!(result, "60");
    }

    #[test]
    fn test_b() {
        let input = "2333133121414131402";
        let result = invoke(input);
        assert_eq!(result, "1928");
    }
}
