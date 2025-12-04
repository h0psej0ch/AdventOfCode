pub fn solve() {
    let file_path = "src/puzzles/puzzle3/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");
    let solver = Solver::new(contents);
    solver.one();
    solver.two();
}

struct Solver {
    batteries: Vec<Vec<usize>>,
}

impl Solver {
    fn new(contents: String) -> Self {
        Self {
            batteries: contents
                .lines()
                .into_iter()
                .map(|line| {
                    line.chars()
                        .map(|char| char.to_digit(10).unwrap() as usize)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>(),
        }
    }

    fn one(&self) {
        println!(
            "Puzzle 3.1: {}",
            self.batteries
                .iter()
                .map(|bank| {
                    let mut total = 0;
                    let mut last_idx = 0;
                    (0..2).for_each(|n| {
                        let mut max = 0;
                        bank.iter()
                            .enumerate()
                            .skip(last_idx)
                            .take(bank.len() - (1 - n))
                            .for_each(|(i, &val)| {
                                if val > max {
                                    max = val;
                                    last_idx = i + 1;
                                }
                            });
                        total = total * 10 + max;
                    });
                    total
                })
                .sum::<usize>()
        );
    }

    fn two(&self) {
        println!(
            "Puzzle 3.2: {}",
            self.batteries
                .iter()
                .map(|bank| {
                    let mut total = 0;
                    let mut last_idx = 0;
                    (0..12).for_each(|n| {
                        let mut max = 0;
                        bank.iter()
                            .enumerate()
                            .skip(last_idx)
                            .take(bank.len() - (11 - n))
                            .for_each(|(i, &val)| {
                                if val > max {
                                    max = val;
                                    last_idx = i + 1;
                                }
                            });
                        total = total * 10 + max;
                    });
                    total
                })
                .sum::<usize>()
        );
    }
}
