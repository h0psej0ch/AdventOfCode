use itertools::Itertools;
use std::collections::{HashMap, HashSet};
pub fn solve() {
    let file_path = "src/puzzles/puzzle8/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    let mut solver = Solver::new(contents);
    solver.one();
    solver.two();
}

struct Solver {
    antennas: HashMap<char, Vec<(i64, i64)>>,
    all_antennas: HashSet<(i64, i64)>,
    antinodes: HashSet<(i64, i64)>,
    height: i64,
    width: i64,
}

impl Solver {
    fn new(contents: String) -> Self {
        let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
        let mut all_antennas: HashSet<(i64, i64)> = HashSet::new();
        let mut antinodes: HashSet<(i64, i64)> = HashSet::new();

        contents.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                if char != '.' {
                    all_antennas.insert((x as i64, y as i64));
                    antennas.entry(char).or_insert_with(Vec::new).push((x as i64, y as i64));
                }
            })
        });

        let height = contents.lines().collect::<Vec<_>>().len() as i64;
        let width = contents.lines().collect::<Vec<_>>()[0].chars().collect::<Vec<_>>().len() as i64;

        // println!("{:?}", all_antennas);

        Self {
            antennas,
            all_antennas,
            antinodes,
            height,
            width,
        }
    }

    fn one(&mut self) {
        self.antennas
            .iter()
            .for_each(|(_key, value)|
                value.iter()
                    .tuple_combinations()
                    .for_each(|((x1, y1), (x2, y2))| {
                        let pos1 = (x1 + (x1 - x2), y1 + (y1 - y2));
                        let pos2 = (x2 + (x2 - x1), y2 + (y2 - y1));
                        if pos1.0 >= 0 && pos1.1 >= 0 && pos1.0 < self.width && pos1.1 < self.height {
                            println!{"{:?}", pos1}
                            self.antinodes.insert(pos1);
                        }
                        if pos2.0 >= 0 && pos2.1 >= 0 && pos2.0 < self.width && pos2.1 < self.height {
                            println!{"{:?}", pos2}
                            self.antinodes.insert(pos2);
                        }
                    }));
        println!("{}", self.antinodes.len());

        for i in 0..12 {
            for j in 0..12 {
                if self.antinodes.contains(&(j as i64, i as i64)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }

    fn two(&mut self) {
        self.antennas
            .iter()
            .for_each(|(_key, value)|
                value.iter()
                    .tuple_combinations()
                    .for_each(|((x1, y1), (x2, y2))| {
                        let dif1 = ((x1 - x2), (y1 - y2));
                        let dif2 = ((x2 - x1), (y2 - y1));
                        let mut pos1 = (*x1,*y1);
                        let mut pos2 = (*x2,*y2);
                        while pos1.0 >= 0 && pos1.1 >= 0 && pos1.0 < self.width && pos1.1 < self.height {
                            println!{"{:?}", pos1}
                            self.antinodes.insert(pos1);
                            pos1 = (pos1.0 + dif1.0, pos1.1 + dif1.1);
                        }
                        while pos2.0 >= 0 && pos2.1 >= 0 && pos2.0 < self.width && pos2.1 < self.height {
                            println!{"{:?}", pos2}
                            self.antinodes.insert(pos2);
                            pos2 = (pos2.0 + dif2.0, pos2.1 + dif2.1);
                        }
                    }));
        println!("{}", self.antinodes.len());

        for i in 0..self.height {
            for j in 0..self.width {
                if self.antinodes.contains(&(j as i64, i as i64)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
}
