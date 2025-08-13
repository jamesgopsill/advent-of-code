use regex::Regex;
use std::fs;
// use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2016/10.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    // bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let bots_re = Regex::new(r"bot (\d+)").unwrap();
    let mut max_bot: usize = 0;
    for cap in bots_re.captures_iter(input) {
        let id = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        if id > max_bot {
            max_bot = id;
        }
    }

    println!("Max Bots: {max_bot}");

    let outputs_re = Regex::new(r"output (\d+)").unwrap();
    let mut max_output: usize = 0;
    for cap in outputs_re.captures_iter(input) {
        let id = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        if id > max_output {
            max_output = id;
        }
    }

    println!("Max Outputs: {max_output}");

    let mut outputs = vec![Output::default(); max_output + 1];
    let mut bots = vec![Bot::default(); max_bot + 1];

    for line in input.lines() {
        println!("{line}");
        if line.starts_with("b") {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let id = tokens[1].parse::<usize>().unwrap();
            let bot = &mut bots[id];
            match tokens[5] {
                "output" => bot.low_kind = Kind::Output,
                "bot" => bot.low_kind = Kind::Bot,
                s => panic!("Whoops, got {s}"),
            };
            bot.low_idx = tokens[6].parse::<usize>().unwrap();
            match tokens[10] {
                "output" => bot.high_kind = Kind::Output,
                "bot" => bot.high_kind = Kind::Bot,
                s => panic!("Whoops, got {s}"),
            };
            bot.high_idx = tokens[11].parse::<usize>().unwrap();
            continue;
        }
        if line.starts_with("v") {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let id = tokens[5].parse::<usize>().unwrap();
            let val = tokens[1].parse::<u32>().unwrap();
            let bot = &mut bots[id];
            bot.chips.push(val);
        }
    }

    // Now to loop until we can't loop anymore.
    loop {
        // println!("Looping");
        let mut idx: Option<usize> = None;
        let mut b: Option<Bot> = None;
        for (i, bot) in bots.iter().enumerate() {
            if bot.chips.len() == 2 {
                b = Some(bot.clone());
                idx = Some(i);
            }
        }
        // println!("{:?}", b);

        if let Some(mut bot) = b {
            bot.chips.sort();
            match bot.low_kind {
                Kind::Bot => bots[bot.low_idx].chips.push(bot.chips[0]),
                Kind::Output => outputs[bot.low_idx].value = Some(bot.chips[0]),
                _ => {}
            }
            match bot.high_kind {
                Kind::Bot => bots[bot.high_idx].chips.push(bot.chips[1]),
                Kind::Output => outputs[bot.high_idx].value = Some(bot.chips[1]),
                _ => {}
            }
            bot.chips.clear();
            bots[idx.unwrap()] = bot;
        } else {
            break;
        }
    }

    let ans = outputs[0].value.unwrap() * outputs[1].value.unwrap() * outputs[2].value.unwrap();

    ans.to_string()
}

#[derive(Debug, Default, Clone)]
pub enum Kind {
    Output,
    Bot,
    #[default]
    None,
}

#[derive(Debug, Default, Clone)]
struct Output {
    pub value: Option<u32>,
}

#[derive(Debug, Default, Clone)]
struct Bot {
    pub low_kind: Kind,
    pub low_idx: usize,
    pub high_kind: Kind,
    pub high_idx: usize,
    pub chips: Vec<u32>,
}
