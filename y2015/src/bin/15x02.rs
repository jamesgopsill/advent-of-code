use itertools::Itertools;

use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2015/15.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut max = 0;
    let mut ingredient_stats: Vec<Vec<i32>> = vec![];
    let mut calories: Vec<i32> = vec![];
    for lines in input.trim().lines() {
        let mut scores: Vec<i32> = vec![];
        let (_, stats) = lines.split_once(":").unwrap();
        let stats = stats.split(",");
        for stat in stats {
            let (_, value) = stat.trim().split_once(" ").unwrap();
            scores.push(value.parse::<i32>().unwrap());
        }
        calories.push(scores.pop().unwrap());
        ingredient_stats.push(scores);
    }
    println!("{:?}", ingredient_stats);
    // Now for the permutations of 100 into n buckets.
    let n_ingredients = ingredient_stats.len();
    let n_categories = ingredient_stats[0].len();
    let combos = (0..=100).permutations(n_ingredients);
    for combo in combos {
        let s: i32 = combo.iter().sum();
        if s != 100 {
            continue;
        }
        // adding the calorie constraint
        let mut cal = 0;
        for (i, c) in calories.iter().enumerate() {
            cal += combo[i] * c;
        }
        if cal != 500 {
            continue;
        }

        // do the calculation.
        let mut category_totals = vec![0; n_categories];
        for (i, ingredient) in ingredient_stats.iter().enumerate() {
            for (j, stat) in ingredient.iter().enumerate() {
                category_totals[j] += stat * combo[i];
            }
        }
        // multiply
        let mut sum: i32 = 1;
        for total in category_totals.iter() {
            if *total > 0 {
                sum *= total;
            } else {
                sum = 0;
            }
        }
        if sum > max {
            max = sum;
        }
    }
    max.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let result = invoke(
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
",
        );
        assert_eq!(result, "57600000");
    }
}
