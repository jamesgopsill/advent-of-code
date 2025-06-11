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

    // Strategy Two

    guard_id = 0;
    let mut max_sleep: u32 = 0;
    let mut minute: u32 = 0;
    for (key, value) in guards.iter() {
        for (i, v) in value.iter().enumerate() {
            if *v > max_sleep {
                guard_id = *key;
                max_sleep = *v;
                minute = i as u32;
            }
        }
    }
    println!("{} {}", guard_id, minute);

    (guard_id * minute).to_string()
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
        assert_eq!(result, "4455");
    }
}
