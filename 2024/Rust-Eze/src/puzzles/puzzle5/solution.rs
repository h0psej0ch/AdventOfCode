use std::collections::HashMap;
use itertools::Itertools;

pub fn solve() {
    let file_path = "src/puzzles/puzzle5/input.txt";

    let contents = std::fs::read_to_string(file_path)
        .expect("No input file");

    let solver = Solver::new(contents);
    solver.one();
    solver.two();

}

struct Solver {
    map: HashMap<usize, Vec<usize>>,
    rules: Vec<Vec<usize>>,
}

impl Solver {
    fn new(contents: String) -> Self {
        let split: Vec<_> = contents.split("\n\n").collect();

        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        split[0].lines()
            .map(|line| {
                let numbers: Vec<_> = line.split("|").map(|num| num.parse::<usize>().unwrap()).collect();
                (numbers[0], numbers[1])
            })
            .for_each(|(key, value)| map.entry(key).or_insert_with(Vec::new).push(value));

        let rules: Vec<Vec<usize>> = split[1].lines().map(|line| line.split(",").map(|string| string.parse::<usize>().unwrap()).collect()).collect();

        Self {
            map,
            rules,
        }
    }
    fn one(&self) {
        let sum = self.rules.iter().
            filter(|numbers|
                numbers.iter().tuple_combinations()
                    .all(|(a, b)| {
                        match self.map.get(b) {
                            Some(vec) => !(vec.contains(a)),
                            None => true,
                        }
                    })
            ).map(|numbers| numbers[(numbers.len() / 2)]).sum::<usize>();
        println!("Puzzle One: {}", sum);
    }

    fn two(&self) {}
}