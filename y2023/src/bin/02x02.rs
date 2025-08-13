use regex::Regex;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/02.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

struct Game {
    r: u32,
    g: u32,
    b: u32,
}

fn invoke(input: &str) -> String {
    let mut out: u32 = 0;

    let lines = input.lines();
    let balls_reg = Regex::new(r"(\d+)\s([brg])").unwrap();

    for line in lines {
        //	dbg!(line);
        let mut game = Game { r: 0, g: 0, b: 0 };
        let caps = balls_reg.captures_iter(line);
        for cap in caps {
            let number = cap[1].parse::<u32>().unwrap();
            let colour = &cap[2];
            match colour {
                "r" => {
                    if number > game.r {
                        game.r = number;
                    }
                }
                "g" => {
                    if number > game.g {
                        game.g = number;
                    }
                }
                "b" => {
                    if number > game.b {
                        game.b = number;
                    }
                }
                _ => {
                    println!("Should not get here!")
                }
            }
        }
        out += game.r * game.g * game.b;
    }
    out.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = invoke(input);
        assert_eq!(result, "2286");
    }
}
