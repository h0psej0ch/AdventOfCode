use std::collections::HashMap;

pub fn solve() {
    let file_path = "src/puzzles/puzzle11/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");
    let s = Solver::new(contents);
    s.one();
    s.two();
}

struct Solver {
    connections: HashMap<String, Vec<String>>,
}

impl Solver {
    fn new(contents: String) -> Self {
        let mut connections = HashMap::new();

        contents
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .for_each(|(id, line)| {
                connections.insert(
                    id.to_string(),
                    line.split_whitespace()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>(),
                );
            });

        Self { connections }
    }
    fn one(&self) {
        println!(
            "Puzzle 11.1: {}",
            self.rec("you", "out", &mut HashMap::new())
        );
    }

    fn two(&self) {
        println!(
            "Puzzle 11.2: {}",
            self.rec("svr", "fft", &mut HashMap::new())
                * self.rec("fft", "dac", &mut HashMap::new())
                * self.rec("dac", "out", &mut HashMap::new())
                + self.rec("svr", "dac", &mut HashMap::new())
                    * self.rec("dac", "fft", &mut HashMap::new())
                    * self.rec("fft", "out", &mut HashMap::new())
        );
    }

    fn rec(&self, current: &str, target: &str, memo: &mut HashMap<String, usize>) -> usize {
        if current == target {
            return 1;
        }

        if let Some(&count) = memo.get(current) {
            return count;
        }

        let mut total = 0;
        if let Some(neighbors) = self.connections.get(current) {
            for neighbor in neighbors {
                total += self.rec(neighbor, target, memo);
            }
        }
        memo.insert(current.to_string(), total);
        total
    }
}
