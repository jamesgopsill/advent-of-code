use chrono::{NaiveDateTime, Timelike};
use std::{collections::HashMap, fs};
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2018/04.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut events: Vec<Event> = Vec::new();
    let mut guards: HashMap<u32, [u32; 60]> = HashMap::new();

    for line in input.lines() {
        // TODO
        let date = chrono::NaiveDateTime::parse_from_str(&line[1..=16], "%Y-%m-%d %H:%M").unwrap();
        let event = Event {
            date,
            comment: &line[19..],
        };
        events.push(event)
    }

    events.sort_by(|a, b| a.date.cmp(&b.date));

    // println!("{:?}", events);

    let mut guard_id: u32 = 0;
    let mut asleep: usize = 0;
    for event in &events {
        // If either are different then we're starting a new day.
        // And it should be a guard comment.

        if event.comment.contains("Guard") {
            let tokens: Vec<&str> = event.comment.split_whitespace().collect();
            let token = tokens[1];
            guard_id = token[1..].parse::<u32>().unwrap();
            guards.entry(guard_id).or_insert([0; 60]);
            asleep = 0;
            continue;
        }

        if event.comment.contains("asleep") {
            asleep = event.date.minute() as usize;
            continue;
        }

        if event.comment.contains("wakes") {
            let mut awake = event.date.minute() as usize;
            // The value has "overflowed" into the next day. I.e., the guard was asleep for the rest of their shift.
            if awake < asleep {
                awake = 60;
            }
            //println!("From {} to {}", asleep, awake);
            let schedule = guards.get_mut(&guard_id).unwrap();
            for minute in schedule[asleep..awake].iter_mut() {
                *minute += 1;
            }
            continue;
        }
        println!("{:?}", event);
        panic!("Should not get here.")
    }

    // println!("{:?}", guards);

    // Strategy One

    let mut guard_most_asleep: u32 = 0;
    let mut max_asleep: u32 = 0;
    //let mut max_minute: u32 = 0;
    for (key, value) in guards.iter() {
        let sum: u32 = value.iter().sum();
        // println!("Guard: {}, Asleep: {}", key, sum);
        if sum > max_asleep {
            max_asleep = sum;
            guard_most_asleep = *key;
        }
    }

    //println!("Guard Most Asleep: {}", guard_most_asleep);

    let mut max_minute: u32 = 0;
    max_asleep = 0;
    let times = guards.get(&guard_most_asleep).unwrap();
    for (i, v) in times.iter().enumerate() {
        if *v > max_asleep {
            max_asleep = *v;
            max_minute = i as u32;
        }
    }

    //println!("Most asleep at: {}", max_minute);

    (guard_most_asleep * max_minute).to_string()
}

#[derive(Debug)]
struct Event<'a> {
    date: NaiveDateTime,
    comment: &'a str,
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";
        let result = invoke(input);
        assert_eq!(result, "240");
    }
}
