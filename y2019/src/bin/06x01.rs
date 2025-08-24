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

    fn path(name: &str, bodies: &HashMap<Box<str>, CelestialBody>, orbits: &mut u64) {
        let body = bodies.get(name).unwrap();
        if let Some(parent) = body.parent.as_ref() {
            *orbits += 1;
            path(parent, bodies, orbits);
        }
    }

    let mut orbits = 0;
    for (k, _) in bodies.iter() {
        path(k, &bodies, &mut orbits);
    }

    orbits.to_string()
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
";
        let result = invoke(input);
        assert_eq!(result, "42");
    }
}
