use std::collections::HashMap;
//use utils::bench;

fn main() {
    let input = include_str!("../../../puzzle_data/2019/06.txt");
    let out = invoke(input);
    println!("{out}");
    //bench(invoke, &input);
}

fn invoke(input: &str) -> String {
    let lines = input.trim().lines();
    let mut bodies: HashMap<Box<str>, CelestialBody> = HashMap::new();
    for line in lines {
        let (left, right) = line.trim().split_once(")").unwrap();
        let left: Box<str> = left.into();
        let right: Box<str> = right.into();
        bodies
            .entry(left.clone())
            .and_modify(|b| b.children.push(right.clone()))
            .or_insert(CelestialBody {
                parent: None,
                children: vec![right.clone()],
            });
        bodies
            .entry(right)
            .and_modify(|b| b.parent = Some(left.clone()))
            .or_insert(CelestialBody {
                parent: Some(left),
                children: vec![],
            });
    }

    fn path_to_com(
        current: &str,
        path: &mut Vec<Box<str>>,
        bodies: &HashMap<Box<str>, CelestialBody>,
    ) {
        path.push(current.into());
        let body = bodies.get(current).unwrap();
        if let Some(parent) = body.parent.as_ref() {
            path_to_com(parent, path, bodies);
        }
    }

    let mut path_one = Vec::new();
    path_to_com("YOU", &mut path_one, &bodies);
    println!("Path One: {path_one:?}");

    let mut path_two = Vec::new();
    path_to_com("SAN", &mut path_two, &bodies);
    println!("Path Two: {path_two:?}");

    let traversals = || {
        for (i, a) in path_one.iter().enumerate() {
            for (j, b) in path_two.iter().enumerate() {
                if a == b {
                    return i + j;
                }
            }
        }
        0
    };
    let t = traversals() - 2;

    format!("{t}")
}

pub struct CelestialBody {
    parent: Option<Box<str>>,
    children: Vec<Box<str>>,
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN
";
        let result = invoke(input);
        assert_eq!(result, "4");
    }
}
