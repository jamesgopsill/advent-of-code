use std::fs;
use utils::bench;

fn main() {
	let input = fs::read_to_string("puzzle_data/2023/23.txt").unwrap();
	let out = invoke(&input);
	println!("{}", out);
	bench(invoke, &input);
}

fn invoke(input: &str) -> String {
	let mut links: Vec<(&str, &str)> = vec![];
	for line in input.lines() {
		let (left, right) = line.split_once("-").unwrap();
		links.push((left, right));
		links.push((right, left));
	}

	println!("{:?}", links);

	let mut cliques = vec![];
	traverse(vec![], &links, &mut cliques);
	println!("Cliques: {}", cliques.len());
	println!("{:?}", cliques);

	let mut ans: u32 = 0;
	for clique in cliques {
		if clique.0.starts_with("t") {
			ans += 1;
			continue;
		}
		if clique.1.starts_with("t") {
			ans += 1;
			continue;
		}
		if clique.2.starts_with("t") {
			ans += 1;
			continue;
		}
	}

	ans.to_string()
}

fn traverse<'a>(
	chain: Vec<&'a str>,
	links: &Vec<(&'a str, &'a str)>,
	cliques: &mut Vec<(&'a str, &'a str, &'a str)>,
) {
	match chain.len() {
		4.. => {
			println!("Should not get here");
		}
		3 => {
			// println!("3 combo: {:?}", chain);
			for (left, right) in links.iter() {
				let first = chain.first().unwrap();
				let last = chain.last().unwrap();
				if left == last && right == first {
					//println!("Clique Found");
					let mut chain = chain.clone();
					chain.sort();
					let clique = (chain[0], chain[1], chain[2]);
					if !cliques.contains(&clique) {
						//println!("New Exists Added");
						cliques.push(clique);
					} else {
						//println!("Clique Exists Already");
					}
				}
			}
		}
		1..=2 => {
			for (left, right) in links.iter() {
				if chain.contains(left) && chain.contains(right) {
					continue;
				}
				let last = chain.last().unwrap();
				if left == last {
					// println!("{:?} left match {} {}", chain, left, right);
					let mut chain = chain.clone();
					chain.push(right);
					traverse(chain, links, cliques);
					continue;
				}
			}
		}
		0 => {
			for (i, (left, right)) in links.iter().enumerate() {
				let mut chain = chain.clone();
				chain.push(left);
				chain.push(right);
				println!("Starting: {} {:?}", i, chain);
				traverse(chain, links, cliques);
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::invoke;

	#[test]
	fn test_a() {
		let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
		let result = invoke(input);
		assert_eq!(result, "7");
	}
}
