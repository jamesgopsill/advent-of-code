use regex::Regex;
use std::collections::HashMap;
use std::fs;
use utils::bench;

fn main() {
    let input = fs::read_to_string("puzzle_data/2023/24.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let mut wires: HashMap<&str, Option<u8>> = HashMap::new();

    // Variables
    let re = Regex::new(r"[\w]{2,10}").unwrap();
    let caps = re.captures_iter(input);
    for cap in caps {
        let var = cap.get(0).unwrap().as_str();
        println!("{}", var);
        if ["AND", "XOR", "OR"].contains(&var) {
            continue;
        }
        //if wires.get(var).is_none() {
        if !wires.contains_key(var) {
            wires.insert(var, None);
        }
    }

    // Values
    let re = Regex::new(r"(\w+):\s(\d)").unwrap();
    let caps = re.captures_iter(input);
    for cap in caps {
        let var = cap.get(1).unwrap().as_str();
        let val = cap.get(2).unwrap().as_str().parse::<u8>().unwrap();
        println!("{} {}", var, val);
        wires.insert(var, Some(val));
    }

    // Instructions
    let mut instructions: Vec<(&str, &str, &str, &str)> = vec![];

    let re = Regex::new(r"(\w+)\s(AND|XOR|OR)\s(\w+)\s->\s(\w+)").unwrap();
    let caps = re.captures_iter(input);
    for cap in caps {
        let lhs = cap.get(1).unwrap().as_str();
        let op = cap.get(2).unwrap().as_str();
        let rhs = cap.get(3).unwrap().as_str();
        let out = cap.get(4).unwrap().as_str();
        println!("{} {} {} {}", lhs, op, rhs, out);
        instructions.push((lhs, op, rhs, out));
    }

    loop {
        // Check if we have calculated all the values.
        let mut flag = true;
        for (_k, v) in wires.iter() {
            if v.is_none() {
                flag = false;
                break;
            }
        }
        if flag {
            break;
        }
        // Find instructions we can operate on
        for (lhs, op, rhs, out) in instructions.iter() {
            // Continue if we have alread calculated the value
            let out_val = wires.get(out).unwrap();
            if out_val.is_some() {
                continue;
            }
            let lhs = wires.get(lhs).unwrap();
            let rhs = wires.get(rhs).unwrap();
            if lhs.is_some() && rhs.is_some() {
                let lhs = lhs.unwrap();
                let rhs = rhs.unwrap();
                match *op {
                    "AND" => {
                        if lhs == 1 && rhs == 1 {
                            wires.insert(out, Some(1));
                        } else {
                            wires.insert(out, Some(0));
                        }
                    }
                    "XOR" => {
                        if lhs != rhs {
                            wires.insert(out, Some(1));
                        } else {
                            wires.insert(out, Some(0));
                        }
                    }
                    "OR" => {
                        if lhs == 1 || rhs == 1 {
                            wires.insert(out, Some(1));
                        } else {
                            wires.insert(out, Some(0));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    //println!("{:?}", wires);

    let mut bits = "".to_string();
    for i in (0..99).rev() {
        let key = format!("z{:0>2}", i);
        // println!("{}", key);
        if let Some(v) = wires.get(key.as_str()) {
            match v {
                Some(0) => bits.push('0'),
                Some(1) => bits.push('1'),
                _ => {}
            }
        }
    }
    println!("{}", bits);
    let digit = isize::from_str_radix(bits.as_str(), 2).unwrap();
    println!("{}", digit);

    digit.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        let result = invoke(input);
        assert_eq!(result, "4");
    }

    #[test]
    fn test_b() {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        let result = invoke(input);
        assert_eq!(result, "2024");
    }
}
