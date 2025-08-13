use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs,
    thread::park,
};
//use utils::bench;

// ###############
// TOFINISH.

fn main() {
    let input = fs::read_to_string("puzzle_data/2017/07.txt").unwrap();
    let out = invoke(&input);
    println!("{}", out);
    //bench(invoke, &input);
}

#[derive(Debug)]
pub struct Program<'a> {
    pid: &'a str,
    weight: u32,
    sum: u32,
    parent: Option<&'a str>,
    children: Vec<&'a str>,
}

impl<'a> Program<'a> {
    pub fn new(pid: &'a str, weight: u32) -> Self {
        Self {
            pid,
            weight,
            sum: weight,
            parent: None,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, pid: &'a str, weight: u32) {
        self.sum += weight;
        self.children.push(pid);
    }
}

#[derive(Debug, Default)]
pub struct Tower<'a> {
    tower: HashMap<&'a str, Program<'a>>,
}

impl<'a> Tower<'a> {
    pub fn add_program(&mut self, pid: &'a str, weight: u32) {
        let program = Program::new(pid, weight);
        self.tower.insert(program.pid, program);
    }

    pub fn associate(&mut self, parent_pid: &'a str, child_pid: &'a str) {
        let weight: u32;
        {
            let child = self.tower.get_mut(child_pid).unwrap();
            child.parent = Some(parent_pid);
            weight = child.weight;
        }
        let parent = self.tower.get_mut(parent_pid).unwrap();
        parent.add_child(child_pid, weight);
    }

    pub fn find_root(&'a self) -> Result<&'a Program<'a>, TowerError> {
        for (_, v) in self.tower.iter() {
            if v.parent.is_none() {
                return Ok(v);
            }
        }
        Err(TowerError::NoRootFound)
    }

    pub fn find_children(&'a self) -> Vec<&'a str> {
        let mut child_pids = vec![];
        for (_, v) in self.tower.iter() {
            if v.children.is_empty() {
                child_pids.push(v.pid.clone())
            }
        }
        child_pids
    }

    pub fn get_program_sum(&mut self, key: &str) -> u32 {
        let p = self.tower.get_mut(key).unwrap();
        p.sum
    }
}

#[derive(Debug)]
pub enum TowerError {
    NoRootFound,
}

fn invoke(input: &str) -> String {
    let mut tower = Tower::default();

    // Create the programs
    for line in input.lines() {
        let program = line
            .split(")")
            .collect_vec()
            .first()
            .unwrap()
            .to_owned()
            .trim();
        let program = program.split("(").collect_vec();
        let pid = program.first().unwrap().to_owned().trim();
        let weight = program.last().unwrap().parse::<u32>().unwrap();
        tower.add_program(pid, weight);
    }

    // Now for the edges
    for line in input.lines() {
        if !line.contains("->") {
            continue;
        }
        let parent_pid = line
            .split("(")
            .collect_vec()
            .first()
            .unwrap()
            .to_owned()
            .trim();
        let children = line.split("->").collect_vec().last().unwrap().to_owned();
        let children = children.split(",").collect_vec();
        for child in children {
            let child_pid = child.trim();
            tower.associate(parent_pid, child_pid);
        }
    }

    let mut nodes: Vec<&Program> = vec![];
    for (_, v) in tower.tower.iter() {
        if v.children.is_empty() {
            let parent = tower.tower.get(v.parent.unwrap()).unwrap();
            nodes.push(parent);
        }
    }
    let mut level = 0;
    loop {
        let mut next_level_nodes: Vec<&Program> = vec![];
        let mut sums: HashSet<u32> = HashSet::new();
        for node in nodes.iter() {
            sums.insert(node.sum);
            let parent = tower.tower.get(node.parent.unwrap()).unwrap();
            next_level_nodes.push(parent);
        }
        if sums.len() > 1 {
            println!("Error Found At Level {}", level);
            println!("{:?}", sums);
            break;
        } else {
            nodes = next_level_nodes;
            level += 1;
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        let result = invoke(input);
        assert_eq!(result, "8");
    }
}

/*

pub struct Program<'a> {
    name: &'a str,
    weight: u64,
    parent: Option<&'a Program<'a>>,
    children: Vec<&'a Program<'a>>,
}

impl<'a> Program<'a> {
    pub fn new(
        name: &'a str,
        weight: u64,
    ) -> Self {
        Self {
            name,
            weight,
            parent: None,
            children: vec![],
        }
    }
}

fn invoke<'a>(input: &str) -> String {
    let mut tower: HashMap<&str, Program> = HashMap::new();

    // Initialise the nodes
    for line in input.lines() {
        let program = line
            .split(")")
            .collect_vec()
            .first()
            .unwrap()
            .to_owned()
            .trim();
        let program = program.split("(").collect_vec();
        let name = program.first().unwrap().to_owned().trim();
        let weight = program.last().unwrap().parse::<u64>().unwrap();
        let p = Program::new(name, weight);
        tower.insert(p.name, p);
    }

    // Now for the edges
    for line in input.lines() {
        if !line.contains("->") {
            continue;
        }
        let parent = line
            .split("(")
            .collect_vec()
            .first()
            .unwrap()
            .to_owned()
            .trim();
        let children = line.split("->").collect_vec().last().unwrap().to_owned();
        let children = children.split(",").collect_vec();
        for child in children {
            let child = child.trim();
            {
                let parent = tower.get(parent).unwrap();
                let child = tower.get_mut(child).unwrap();
                child.parent = Some(parent);
            }
            {
                let parent = tower.get_mut(parent).unwrap();
                let child = tower.get(child).unwrap();
                parent.children.push(child);
            }
        }
    }

    // Now find the root node
    let mut level: Vec<&'a Program<'a>> = vec![];
    for (_, v) in tower.iter() {
        if v.parent.is_none() {
            level = v.children;
        }
    }

    loop {
        let res = check_level(&level);
        match res {
            Ok(lvl) => {
                if lvl.is_empty() {
                    level = lvl
                } else {
                    return "".to_string();
                }
            }
            Err(diff) => {
                println!("Error Found");
                return diff.to_string();
            }
        }
    }

    return "".to_string();
}

fn check_level<'a>(programs: &'a Vec<&'a Program<'a>>) -> Result<Vec<&'a Program<'a>>, i64> {
    let mut sub_level_programs = vec![];
    let mut weights = HashSet::new();
    for p in programs {
        let mut sum = p.weight;
        for c in p.children.iter() {
            sum += c.weight;
            sub_level_programs.push(c.to_owned());
        }
        weights.insert(sum);
    }
    match weights.len() {
        3.. => panic!("Shouldn't have more than one difference"),
        2 => {
            let weights = weights.into_iter().collect_vec();
            let diff = weights[0] as i64 - weights[1] as i64;
            Err(diff)
        }
        1 => Ok(sub_level_programs),
        0 => panic!("Shouldn't have zero"),
    }
}

#[cfg(test)]
mod tests {
    use super::invoke;

    #[test]
    fn test_a() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        let result = invoke(input);
        assert_eq!(result, "7");
    }
}

*/
