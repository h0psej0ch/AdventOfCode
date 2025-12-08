use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve() {
    let file_path = "src/puzzles/puzzle7/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");
    let s = Solver::new(contents);
    s.one();
    s.two();
}

struct Solver {
    origin: (usize, usize),
    splitters: HashSet<(usize, usize)>,
    end: usize,
}

impl Solver {
    fn new(contents: String) -> Self {
        Self {
            origin: contents
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|&(_, char)| char == 'S')
                        .map(move |(x, _)| (x, y))
                })
                .next()
                .expect("No 'S' found in input"),
            splitters: contents
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|&(_, char)| char == '^')
                        .map(move |(x, _)| (x, y))
                })
                .collect::<HashSet<(usize, usize)>>(),
            end: contents.lines().count(),
        }
    }
    fn one(&self) {
        let mut splits = 0;
        let mut beams: HashSet<(usize, usize)> = HashSet::new();
        beams.insert(self.origin);
        for _ in 0..self.end {
            let mut new_beams: HashSet<(usize, usize)> = HashSet::new();
            for ((x, y)) in beams {
                if self.splitters.contains(&(x, y)) {
                    splits += 1;
                    new_beams.insert((x - 1, y + 1));
                    new_beams.insert((x + 1, y + 1));
                } else {
                    new_beams.insert((x, y + 1));
                }
            }
            beams = new_beams;
        }

        println!("Puzzle 7.1: {}", splits);
    }

    fn two(&self) {
        let mut beams: HashMap<(usize, usize), usize> = HashMap::new();
        beams.insert(self.origin, 1);
        for _ in 0..self.end {
            let mut new_beams: HashMap<(usize, usize), usize> = HashMap::new();
            for ((x, y), n) in beams {
                if self.splitters.contains(&(x, y)) {
                    new_beams
                        .entry((x - 1, y + 1))
                        .and_modify(|m| *m += n)
                        .or_insert(n);
                    new_beams
                        .entry((x + 1, y + 1))
                        .and_modify(|m| *m += n)
                        .or_insert(n);
                } else {
                    new_beams
                        .entry((x, y + 1))
                        .and_modify(|m| *m += n)
                        .or_insert(n);
                }
            }
            beams = new_beams;
        }

        println!("Puzzle 7.2: {}", beams.values().sum::<usize>());
    }
}
