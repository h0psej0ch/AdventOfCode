use std::collections::HashSet;

pub fn solve() {
    let file_path = "src/puzzles/puzzle25/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(contents);
}

fn one(contents: String) {
    let mut keys: HashSet<Vec<usize>> = HashSet::new();
    let mut locks: HashSet<Vec<usize>> = HashSet::new();
    contents
        .split("\n\n")
        .map(|input| {
            input
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<Vec<_>>>()
        })
        .for_each(|pos| {
            let mut values = Vec::new();
            if pos[0][0] == '#' {
                for i in 0..5 {
                    let mut j = 1;
                    while pos[j][i] == '#' {
                        j += 1;
                    }
                    values.push(j - 1);
                }
                locks.insert(values);
            } else {
                for i in 0..5 {
                    let mut j = 1;
                    while pos[pos.len() - j][i] == '#' {
                        j += 1;
                    }
                    values.push(j - 2);
                }
                keys.insert(values);
            }
        });

    let result = keys
        .iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| {
                    for i in 0..5 {
                        if key[i] + lock[i] > 5 {
                            return false;
                        }
                    }
                    true
                })
                .count()
        })
        .sum::<usize>();

    println!("Puzzle: {}", result);
}
