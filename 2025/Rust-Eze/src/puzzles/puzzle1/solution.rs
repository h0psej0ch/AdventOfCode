pub fn solve() {
    let file_path = "src/puzzles/puzzle1/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    let s = Solver::new(contents);
    s.one();
    s.two();
}

struct Solver {
    instructions: Vec<(bool, usize)>,
}

impl Solver {
    fn new(contents: String) -> Self {
        Self {
            instructions: contents
                .lines()
                .map(|line: &str| {
                    let mut chars = line.chars();
                    let first = chars.next().unwrap();
                    let rest: String = chars.collect();
                    (first == 'R', rest.parse().unwrap())
                })
                .collect(),
        }
    }
    fn one(&self) {
        let mut dial: isize = 50;
        let total = self
            .instructions
            .iter()
            .filter(|(right, length)| {
                dial += if *right {
                    *length as isize
                } else {
                    -(*length as isize)
                };
                dial %= 100;
                dial == 0
            })
            .count();
        println!("Puzzle 1.1: {}", total)
    }

    fn two(&self) {
        let mut dial: isize = 50;
        let total: isize = self
            .instructions
            .iter()
            .map(|(right, length)| {
                dial += if *right {
                    *length as isize
                } else {
                    -(*length as isize) + if dial == 0 { 100 } else { 0 }
                };
                if dial <= 0 {
                    dial -= 100;
                }
                let ret = (dial / 100).abs();
                dial %= 100;
                if dial < 0 {
                    dial += 100;
                }
                ret
            })
            .sum();
        println!("Puzzle 1.2: {}", total);
    }
}
