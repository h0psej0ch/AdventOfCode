use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let file_path = "src/puzzles/puzzle23/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    contents
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .for_each(|(pc1, pc2)| {
            connections.entry(pc1).or_insert(Vec::new()).push(pc2);
            connections.entry(pc2).or_insert(Vec::new()).push(pc1);
        });

    let mut sets: HashSet<Vec<&str>> = HashSet::new();

    connections.iter().for_each(|(&key, value)| {
        if key[0..1].eq("t") {
            value.iter().combinations(2).for_each(|combinations| {
                if connections
                    .get(combinations[0])
                    .unwrap()
                    .contains(combinations[1])
                {
                    let mut set = vec![combinations[0], combinations[1], key];
                    set.sort();
                    sets.insert(set);
                }
            });
        }
    });

    println!("Puzzle One: {:?}", sets.len())
}

fn two(contents: &String) {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    contents
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .for_each(|(pc1, pc2)| {
            connections
                .entry(pc1.to_string())
                .or_insert(Vec::new())
                .push(pc2.to_string());
            connections
                .entry(pc2.to_string())
                .or_insert(Vec::new())
                .push(pc1.to_string());
        });

    let mut computers = HashSet::new();
    connections.keys().for_each(|key| {
        computers.insert(key.clone());
    });

    let mut sets: HashSet<Vec<String>> = HashSet::new();

    bron_kerbosch(
        HashSet::new(),
        computers,
        HashSet::new(),
        &connections,
        &mut sets,
    );

    let longest = sets.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();
    let result = longest.join(",");
    println!("Puzzle Two: {}", result)
}

fn bron_kerbosch(
    r: HashSet<String>,
    mut p: HashSet<String>,
    mut x: HashSet<String>,
    connections: &HashMap<String, Vec<String>>,
    sets: &mut HashSet<Vec<String>>,
) {
    if p.is_empty() && x.is_empty() {
        let mut vector: Vec<String> = r.iter().cloned().collect();
        vector.sort();
        sets.insert(vector);
    } else {
        for v in p.clone() {
            let mut v_set = HashSet::new();
            let mut neighbours = HashSet::new();
            v_set.insert(v.clone());
            connections.get(&v).unwrap().iter().for_each(| neighbour| { neighbours.insert(neighbour.clone()); });
            bron_kerbosch(
                r.union(&v_set).cloned().collect(),
                p.intersection(&neighbours).cloned().collect(),
                x.intersection(&neighbours).cloned().collect(),
                connections,
                sets,
            );

            p.remove(&v);
            x.insert(v);
        }
    }
}
