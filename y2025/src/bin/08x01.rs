use std::collections::HashSet;

fn main() {
    let input = include_str!("../../../puzzle_data/2025/08.txt").trim();
    let out = invoke(input, 1_000);
    println!("{out}");
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Node(i64, i64, i64);

#[derive(Debug)]
pub struct Edge<'a>(&'a Node, &'a Node, i64, u64);

fn invoke(input: &str, cutoff: usize) -> String {
    // Gather the nodes
    let mut nodes: Vec<Node> = Vec::new();
    for line in input.lines() {
        let mut coords = line.split(",");
        let x: i64 = coords
            .next()
            .expect("should be a val")
            .parse()
            .expect("should be parsable");
        let y: i64 = coords
            .next()
            .expect("should be a val")
            .parse()
            .expect("should be parsable");
        let z: i64 = coords
            .next()
            .expect("should be a val")
            .parse()
            .expect("should be parsable");
        nodes.push(Node(x, y, z));
    }

    // Create the complete edge list.
    let mut edges: Vec<Edge> = Vec::new();
    for (i, u) in nodes.iter().enumerate() {
        for v in nodes[i + 1..].iter() {
            let dist = (u.0 - v.0).pow(2) + (u.1 - v.1).pow(2) + (u.2 - v.2).pow(2);
            // let dist = dist.sqrt(); no need as while the diff is wrong the order won't be.
            edges.push(Edge(u, v, dist, 0));
        }
    }

    // Sort the edges by their distance
    edges.sort_by_key(|e| e.2);
    // Choose the n edges.
    let edges = &mut edges[..cutoff];

    println!("Got Here");

    // Now to loop through and group the edges.
    let mut group = 0;
    let mut nsets: Vec<Vec<&Node>> = Vec::new();
    let mut nset: HashSet<&Node> = HashSet::new();
    loop {
        let mut grouping = false;
        // Check for an edge that has not been grouped.
        for e in edges.iter_mut() {
            if e.3 == 0 {
                println!("Grouping on: {e:?}");
                nset.clear();
                group += 1;
                e.3 = group;
                grouping = true;
                nset.insert(e.0);
                nset.insert(e.1);
                break;
            }
        }

        // If we've entered grouping mode.
        if grouping {
            loop {
                let mut finished = true;
                for e in edges.iter_mut() {
                    println!("Checking; {e:?}");
                    if e.3 == 0 && (nset.contains(e.0) || nset.contains(e.1)) {
                        println!("Adding: {e:?}");
                        finished = false;
                        nset.insert(e.0);
                        nset.insert(e.1);
                        e.3 = group;
                    }
                }
                if finished {
                    let ns: Vec<&Node> = nset.iter().cloned().collect();
                    nsets.push(ns);
                    break;
                }
            }
        } else {
            break;
        }
    }

    println!("NodeSets: {:?}", nsets);
    nsets.sort_by_key(|v| usize::MAX - v.len());
    let mut total = 1;
    for nset in &nsets[..3] {
        println!("Cluster size: {}", nset.len());
        total *= nset.len()
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let result = invoke(input, 10);
        assert_eq!(result, "40");
    }
}
