use std::collections::HashMap;

const CARD_RANKS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug)]
struct Bet {
    // hand: String,
    cards: Vec<usize>,
    bet: u32,
    hand_type: u32,
}

impl Bet {
    pub fn new(s: &str) -> Self {
        let elements: Vec<&str> = s.split_whitespace().collect();
        let hand = elements[0].to_string();
        let bet = elements[1].parse::<u32>().unwrap();

        // A map to help work out the hand type
        // Based on card occurrence.
        let mut map: HashMap<char, u32> = HashMap::new();
        for char in CARD_RANKS {
            map.insert(char, 0);
        }

        // Get the vec of cards
        let mut cards: Vec<usize> = vec![];
        for char in hand.chars() {
            let idx = CARD_RANKS.iter().position(|&r| r == char).unwrap();
            cards.push(idx);
            let count = map.get(&char).unwrap();
            map.insert(char, count + 1);
        }

        // Work out the hand type.
        let mut hand_type: u32 = 0;
        let mut twos = 0;
        let mut threes = 0;
        for (_, value) in map.into_iter() {
            if value == 5 {
                hand_type = 6;
                break;
            }
            if value == 4 {
                hand_type = 5;
                break;
            }
            if value == 3 {
                threes += 1;
                continue;
            }
            if value == 2 {
                twos += 1;
                break;
            }
        }
        if threes == 1 && twos == 1 {
            hand_type = 4;
        } else if threes == 1 {
            hand_type = 3;
        } else if twos == 2 {
            hand_type = 2;
        } else if twos == 1 {
            hand_type = 1;
        }

        Self {
            // hand,
            cards,
            bet,
            hand_type,
        }
    }
}

pub fn part_01(input: String) -> u32 {
    let mut sum: u32 = 0;
    let mut bets: Vec<Bet> = vec![];
    let lines = input.lines();
    for line in lines {
        let bet = Bet::new(line);
        bets.push(bet);
    }
    println!("{:?}", bets);

    // Sort (in ascending order) by cards and their appearance
    bets.sort_by(|a, b| a.cards[4].cmp(&b.cards[4]));
    bets.sort_by(|a, b| a.cards[3].cmp(&b.cards[3]));
    bets.sort_by(|a, b| a.cards[2].cmp(&b.cards[2]));
    bets.sort_by(|a, b| a.cards[1].cmp(&b.cards[1]));
    bets.sort_by(|a, b| a.cards[0].cmp(&b.cards[0]));

    // Sort by the hand type.
    bets.sort_by(|a, b| a.hand_type.cmp(&b.hand_type));

    println!("{:?}", bets);

    for (i, bet) in bets.iter().enumerate() {
        let winnings = (i + 1) as u32 * bet.bet;
        sum += winnings;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_01() {
        let input = fs::read_to_string("src/a07/input_01.txt")
            .expect("Should have been able to read the file");
        let result = part_01(input);
        assert_eq!(result, 6440);
    }
}
