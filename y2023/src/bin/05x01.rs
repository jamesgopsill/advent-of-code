use regex::Regex;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/05.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

#[derive(Clone, Debug)]
struct MapRange {
    from_lower: u64,
    from_upper: u64,
    to_lower: u64,
}

fn invoke(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let numbers_re = Regex::new(r"\d+").unwrap();
    let seeds: Vec<u64> = numbers_re
        .find_iter(lines[0])
        .map(|f| f.as_str().parse::<u64>().unwrap())
        .collect();
    println!("Seeds: {:?}", seeds);

    let mut maps: Vec<Vec<MapRange>> = vec![];
    let mut map: Vec<MapRange> = vec![];
    for line in lines[2..].iter() {
        if line.contains("map") {
            map.clear();
            continue;
        }
        if line.is_empty() {
            maps.push(map.clone());
            continue;
        }
        let range: Vec<u64> = numbers_re
            .find_iter(line)
            .map(|f| f.as_str().parse::<u64>().unwrap())
            .collect();
        println!("{:?}", line);
        println!("{:?}", range);
        let map_range = MapRange {
            from_lower: range[1],
            to_lower: range[0],
            from_upper: range[1] + range[2] - 1,
        };
        map.push(map_range);
        println!("{}", line)
    }
    maps.push(map.clone());

    //println!("{:?}", maps);
    let mut loc: u64 = 999_999_999;
    for seed in seeds {
        let mut location = seed;
        for map in &maps {
            for rng in map {
                if rng.from_lower <= location && location <= rng.from_upper {
                    location = rng.to_lower + (location - rng.from_lower);
                    break;
                }
            }
        }
        println!("From: {}, To: {}", seed, location);
        if location < loc {
            loc = location
        }
    }

    loc.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = invoke(input);
        assert_eq!(result, "35");
    }
}
