use std::collections::{HashMap, HashSet};

pub fn solve() {
    let file_path = "src/puzzles/puzzle19/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    let mut solver = Solver::new(contents);

    solver.one();
    solver.two();
}

struct Solver {
    patterns: Vec<String>,
    robes: HashMap<char, Vec<String>>,
    valid: HashSet<String>,
}

impl Solver {
    fn new(contents: String) -> Self {
        let (robes, patterns) = contents.split_once("\n\n").unwrap();
        let robes = robes
            .split(", ")
            .map(|robe| (robe.chars().next().unwrap(), robe.to_string()))
            .fold(HashMap::new(), |mut acc, (key, value)| {
                acc.entry(key).or_insert_with(Vec::new).push(value);
                acc
            });

        let patterns = patterns
            .lines()
            .map(|pattern| pattern.to_string())
            .collect::<Vec<_>>();

        let valid = HashSet::new();

        Self {
            patterns,
            robes,
            valid,
        }
    }

    fn one(&mut self) {
        let mut memo: HashMap<String, bool> = HashMap::new();
        self.valid = self
            .patterns
            .iter()
            .map(|string| string.clone())
            .filter(|pattern| check_pattern(&pattern, &self.robes, &mut memo))
            .collect::<HashSet<String>>();
        println!("Puzzle One: {}", self.valid.len())
    }

    fn two(&mut self) {
        let mut memo: HashMap<String, i64> = HashMap::new();
        let count = self.valid.iter().map(|pattern| possible_patterns(pattern, &self.robes, &mut memo)).sum::<i64>();
        println!("Puzzle Two: {}", count)
    }
}

fn check_pattern(
    pattern: &String,
    robes: &HashMap<char, Vec<String>>,
    memo: &mut HashMap<String, bool>,
) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if let Some(&value) = memo.get(pattern) {
        return value;
    }
    let mut to_check: HashSet<String> = HashSet::new();
    if let Some(to_match) = robes.get(&pattern.chars().next().unwrap()) {
        to_match.iter().for_each(|robe| {
            if pattern.starts_with(robe.as_str()) {
                to_check.insert((&pattern[robe.len()..pattern.len()]).parse().unwrap());
            }
        });
    }
    let result = to_check
        .iter()
        .any(|new_pattern| check_pattern(&new_pattern, robes, memo));
    memo.insert(pattern.to_string(), result);
    result
}

fn possible_patterns(
    pattern: &String,
    robes: &HashMap<char, Vec<String>>,
    memo: &mut HashMap<String, i64>,
) -> i64 {
    if pattern.is_empty() {
        return 1
    }

    if let Some(&value) = memo.get(pattern) {
        return value
    }

    let mut to_check: HashSet<String> = HashSet::new();
    if let Some(to_match) = robes.get(&pattern.chars().next().unwrap()) {
        to_match.iter().for_each(|robe| {
            if pattern.starts_with(robe.as_str()) {
                to_check.insert((&pattern[robe.len()..pattern.len()]).parse().unwrap());
            }
        });
    }
    let result = to_check
        .iter()
        .map(|new_pattern| possible_patterns(&new_pattern, robes, memo)).sum();
    memo.insert(pattern.to_string(), result);
    result
}
