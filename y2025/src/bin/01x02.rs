use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2025/01.txt").trim();
    let out = invoke(input);
    println!("{out}");
    bench(invoke, input);
}

fn invoke(input: &str) -> String {
    // Initialise our password and position
    let mut pwd: i32 = 0;
    let mut pos: i32 = 50;
    // Iterate through the lines
    let lines = input.trim().lines();
    for line in lines {
        // Parse the rotation
        let rot: i32 = line[1..].parse().unwrap();
        // Calculate the quotient where the quotient
        // is how many times we will pass by 0.
        let quo = rot.div_euclid(100).abs();
        pwd += quo;
        // Calculate the remainder and add this to the pos.
        let rem = rot % 100;
        match line.chars().next().unwrap() {
            'L' => pos += rem,
            'R' => pos -= rem,
            _ => panic!("Should not get here"),
        }
        // If we land on 0 then add one
        if pos == 0 {
            pwd += 1
        } else {
            // Check if we have past over a 0 bound.
            let quo = pos.div_euclid(100).abs();
            pwd += quo;
            // Set the remainder as our current position.
            pos %= 100;
        }
    }
    pwd.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let result = invoke(input);
        assert_eq!(result, "6");
    }
}
