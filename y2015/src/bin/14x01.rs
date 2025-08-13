use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle_data/2015/14.txt").unwrap();
    let out = invoke(&input, 2503);
    println!("{}", out);
}

fn invoke(input: &str, seconds: u32) -> String {
    let re = Regex::new(r"(\d+)[a-z\s/]+(\d+)[a-z\s,]+(\d+)").unwrap();
    let captures = re.captures_iter(input.trim());
    let mut max_distance: u32 = 0;
    for reindeer in captures {
        let speed = reindeer.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let flying = reindeer.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let resting = reindeer.get(3).unwrap().as_str().parse::<u32>().unwrap();
        println!("{} {} {}", speed, flying, resting);

        let mut action = Action::Flying;
        let mut time_remaining = seconds;
        let mut distance: u32 = 0;
        loop {
            match action {
                Action::Flying => {
                    if flying > time_remaining {
                        let diff = flying - time_remaining;
                        distance += speed * diff;
                        break;
                    } else {
                        time_remaining -= flying;
                        distance += speed * flying;
                        action = Action::Resting;
                    }
                }
                Action::Resting => {
                    if resting > time_remaining {
                        break;
                    } else {
                        time_remaining -= resting;
                        action = Action::Flying;
                    }
                }
            }
        }
        println!("{}", distance);
        if distance > max_distance {
            max_distance = distance;
        }
    }
    max_distance.to_string()
}

enum Action {
    Flying,
    Resting,
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let result = invoke(
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
",
            1_000,
        );
        assert_eq!(result, "1120");
    }
}
