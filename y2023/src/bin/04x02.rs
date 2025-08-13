use std::fs;
use std::mem::swap;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/04.txt").unwrap();
    let out = invoke(&input);
    println!("{out}");
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut card_map: Vec<Vec<usize>> = vec![];

    let mut current_pile: Vec<usize> = vec![];
    let mut new_pile: Vec<usize> = vec![];

    let mut winning_count: u32;
    let cards = input.lines();
    for (i, card) in cards.enumerate() {
        let mut card_numbers = card.split(":").last().unwrap().split("|");
        let winning_numbers: Vec<i32> = card_numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap())
            .collect();
        let my_numbers: Vec<i32> = card_numbers
            .next()
            .unwrap()
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap())
            .collect();
        winning_count = 0;
        for wn in &winning_numbers {
            if my_numbers.contains(wn) {
                winning_count += 1;
            }
        }

        // println!("---");
        // dbg!(winning_numbers, my_numbers, winning_count);

        current_pile.push(i);
        let map: Vec<usize> = (i + 1..i + winning_count as usize + 1).collect();
        card_map.push(map);
    }

    let mut count: u32 = 0;
    while current_pile.is_empty() {
        for card in &current_pile {
            count += 1;
            let new_cards = &card_map[*card];
            new_pile.extend(new_cards);
        }
        swap(&mut current_pile, &mut new_pile);
        new_pile.clear();
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = invoke(input);
        assert_eq!(result, "30");
    }
}
