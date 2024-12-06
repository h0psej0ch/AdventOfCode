use std::collections::HashSet;

pub fn solve() {
    let file_path = "src/puzzles/puzzle6/input.txt";

    let contents = std::fs::read_to_string(file_path)
        .expect("No input file");

    let mut solver = Solver::new(contents);

    solver.one();
    solver.two();
}

struct Solver {
    guard_map: Vec<Vec<char>>,
    visitations: HashSet<(i32, i32)>,
    start: (i32, i32)
}

impl Solver {

    fn new(contents: String) -> Self {
        let guard_map: Vec<Vec<_>> = contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect();
        // println!("{:?}", guard_map);
        let start: (i32, i32) = guard_map.iter().enumerate()
            .filter(|(_index, line)|
                line.contains(&'^'))
            .map(|(index, line)|
                (index as i32, line.iter()
                    .position(|t| *t == '^').unwrap() as i32)).collect::<Vec<_>>()[0];
        let visitations: HashSet<(i32, i32)> = HashSet::new();

        Self {
            guard_map,
            visitations,
            start
        }

    }
    fn one(&mut self) {

        let mut direction: (i32, i32) = (-1, 0);

        self.visitations.insert(self.start);
        let mut position = self.start.clone();
        while position.0 > 0 && position.1 > 0 && position.0 < self.guard_map.len() as i32 - 1 && position.1 < self.guard_map[0].len() as i32 - 1 {
            if self.guard_map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] == '#' {
                direction = self.turn(direction);
            }
            position = (position.0 + direction.0, position.1 + direction.1);
            self.visitations.insert(position);
        }
        println!("Puzzle One: {}", self.visitations.len())
    }

    fn turn(&self, direction: (i32, i32)) -> (i32, i32) {
        match direction {
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            _ => (0, 0)
        }
    }

    fn two(&mut self) {
        self.visitations.remove(&(self.start));
        let blocks = self.visitations.iter()
            .filter(|visit| {
                let mut new_map = self.guard_map.clone();
                new_map[visit.0 as usize][visit.1 as usize] = '#';
                let result = self.check_loop(new_map);
                result
            }).count();
        println!("Puzzle Two: {}", blocks)
    }

    fn check_loop(&self, new_map: Vec<Vec<char>>) -> bool {
        let mut loop_check: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
        let mut position = self.start;
        let mut direction = (-1, 0);

        while position.0 > 0 && position.1 > 0 && position.0 < new_map.len() as i32 -1  && position.1 < new_map[0].len() as i32 - 1 {
            while new_map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] == '#' {
                direction = self.turn(direction);
            }

            position = (position.0 + direction.0, position.1 + direction.1);

            if loop_check.contains(&(position, direction)) {
                return true
            }

            loop_check.insert((position, direction));
        }

        false
    }
}