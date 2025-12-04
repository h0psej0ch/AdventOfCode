pub fn solve() {
    let file_path = "src/puzzles/puzzle4/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");
    let mut s = Solver::new(contents);
    s.one();
    s.two();
}

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (1, 0),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

struct Solver {
    roll_map: Vec<Vec<bool>>,
}

impl Solver {
    fn new(contents: String) -> Self {
        Self {
            roll_map: contents
                .lines()
                .map(|line| line.chars().map(|char| char == '@').collect())
                .collect(),
        }
    }
    fn one(&self) {
        let count: usize = self
            .roll_map
            .iter()
            .enumerate()
            .map(|(y, roll_line)| {
                roll_line
                    .iter()
                    .enumerate()
                    .filter(|(x, &roll)| {
                        roll && DIRECTIONS
                            .iter()
                            .filter(|(dx, dy)| {
                                let nx = *x as isize + dx;
                                let ny = y as isize + dy;
                                nx >= 0
                                    && ny >= 0
                                    && nx < self.roll_map[0].len() as isize
                                    && ny < self.roll_map.len() as isize
                                    && self.roll_map[ny as usize][nx as usize]
                            })
                            .count()
                            < 4
                    })
                    .count()
            })
            .sum();
        println!("Puzzle 4.1: {}", count);
    }

    fn two(&mut self) {
        let mut changed = true;
        let mut total = 0;
        while changed {
            let count: usize = self
                .roll_map
                .clone()
                .into_iter()
                .enumerate()
                .map(|(y, roll_line)| {
                    roll_line
                        .iter()
                        .enumerate()
                        .filter(|(x, &roll)| {
                            if roll
                                && DIRECTIONS
                                    .iter()
                                    .filter(|(dx, dy)| {
                                        let nx = *x as isize + dx;
                                        let ny = y as isize + dy;
                                        nx >= 0
                                            && ny >= 0
                                            && nx < self.roll_map[0].len() as isize
                                            && ny < self.roll_map.len() as isize
                                            && self.roll_map[ny as usize][nx as usize]
                                    })
                                    .count()
                                    < 4
                            {
                                self.roll_map[y][*x] = false;
                                return true;
                            }
                            false
                        })
                        .count()
                })
                .sum();
            total += count;
            changed = count > 0;
        }
        println!("Puzzle 4.2: {}", total);
    }
}
