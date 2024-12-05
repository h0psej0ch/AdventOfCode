use std::collections::HashMap;
use itertools::Itertools;

pub fn solve() {
    let file_path = "src/puzzles/puzzle5/input.txt";

    let contents = std::fs::read_to_string(file_path)
        .expect("No input file");

    let mut solver = Solver::new(contents);
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
            ).map(|numbers| numbers[numbers.len() / 2]).sum::<usize>();
        println!("Puzzle One: {}", sum);
    }

    fn two(&mut self) {
        let sum: Vec<_> = self.rules.iter().
            filter(|numbers|
                numbers.iter().tuple_combinations()
                    .any(|(a, b)| {
                        match self.map.get(b) {
                            Some(vec) => vec.contains(a),
                            None => false,
                        }
                    })
            ).collect();

        let sum = sum.into_iter().map(|numbers| {
            println!("Numbers: {:?}", numbers);
            let mut changed = true;
            let mut numbers = numbers.clone();
            while changed {
                changed = false;
                let mut new_numbers = numbers.clone();
                numbers.iter().enumerate().tuple_combinations().for_each(|(a, b)| {
                    match self.map.get_mut(b.1) {
                        Some(vec) => {
                            if vec.contains(a.1) {
                                new_numbers.swap(a.0, b.0);
                                println!("Swapped {}, {}", a.1, b.1);
                                changed = true;
                            }
                        }
                        None => {}
                    };
                });
                numbers = new_numbers;
            }
            numbers
        }).map(|numbers| numbers[numbers.len() / 2]).sum::<usize>();
        println!("Puzzle Two: {}", sum);
    }
}