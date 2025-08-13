use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("puzzle_data/2015/14.txt").unwrap();
    let out = invoke(&input, 2503);
    println!("{}", out);
}

fn invoke(input: &str, seconds: usize) -> String {
    let re = Regex::new(r"(\d+)[a-z\s/]+(\d+)[a-z\s,]+(\d+)").unwrap();
    let captures = re.captures_iter(input.trim());
    let mut distances: Vec<u32> = vec![0; seconds];
    let mut runners: Vec<Vec<usize>> = vec![vec![]; seconds];
    let mut scores: Vec<u32> = vec![];
    for (n, reindeer) in captures.enumerate() {
        scores.push(0);
        let speed = reindeer.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let flying = reindeer.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let resting = reindeer.get(3).unwrap().as_str().parse::<u32>().unwrap();
        println!("{} {} {}", speed, flying, resting);

        // TODO: Revise the scoring
        let mut distance = 0;
        let mut action = Action::Flying;
        let mut toggle = flying;
        for i in 0..seconds {
            // Check if I need to change stance
            if toggle == 0 {
                match action {
                    Action::Flying => {
                        action = Action::Resting;
                        toggle = resting;
                    }
                    Action::Resting => {
                        action = Action::Flying;
                        toggle = flying
                    }
                }
            }
            // Update distance
            match action {
                Action::Flying => distance += speed,
                Action::Resting => {}
            }
            // Is it the same or better than a previous score
            if distance == distances[i] {
                runners[i].push(n);
            } else if distance > distances[i] {
                runners[i].clear();
                runners[i].push(n);
                distances[i] = distance;
            }
            //
            toggle -= 1;
        }
    }
    // Score each runner
    for leaders in runners {
        for leader in leaders {
            scores[leader] += 1;
        }
    }
    println!("{:?}", scores);
    scores.iter().max().unwrap().clone().to_string()
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
        assert_eq!(result, "689");
    }
}
