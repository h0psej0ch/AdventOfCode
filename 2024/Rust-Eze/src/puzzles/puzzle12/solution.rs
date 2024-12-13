use std::collections::VecDeque;

const DELTAS: [isize; 2] = [-1, 1];

pub fn solve() {
    let file_path = "src/puzzles/puzzle12/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    let mut solver = Solver::new(contents.clone());
    solver.one();
    let mut solver = Solver::new(contents);
    solver.two();
}

struct Solver {
    characters: Vec<Vec<char>>,
    possibilities: Vec<(usize, usize)>,
    height: usize,
    width: usize,
}

impl Solver {
    fn new(contents: String) -> Self {
        let characters = contents
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut possibilities: Vec<(usize, usize)> = Vec::new();

        let height = characters.len();
        let width = characters[0].len();

        for y in 0..height {
            for x in 0..width {
                possibilities.push((y, x));
            }
        }

        Solver {
            characters,
            possibilities,
            height,
            width,
        }
    }
    fn one(&mut self) {
        let mut total = 0;
        while self.possibilities.len() > 0 {
            let (y, x) = self.possibilities.remove(0);
            let char = self.characters[y][x];

            let mut parameter = 0;
            let mut area = 0;

            let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
            frontier.push_back((y, x));

            while frontier.len() > 0 {
                let (y, x) = frontier.pop_front().unwrap();
                area += 1;
                let mut sides = 4;
                if y < self.height - 1 && char == self.characters[y + 1][x] {
                    sides -= 1;
                    if self.possibilities.contains(&(y + 1, x)) {
                        frontier.push_back((y + 1, x));
                        self.possibilities.retain(|&r| r != (y + 1, x));
                    }
                }
                if y > 0 && char == self.characters[y - 1][x] {
                    sides -= 1;
                    if self.possibilities.contains(&(y - 1, x)) {
                        frontier.push_back((y - 1, x));
                        self.possibilities.retain(|&r| r != (y - 1, x));
                    }
                }
                if x < self.width - 1 && char == self.characters[y][x + 1] {
                    sides -= 1;
                    if self.possibilities.contains(&(y, x + 1)) {
                        frontier.push_back((y, x + 1));
                        self.possibilities.retain(|&r| r != (y, x + 1));
                    }
                }
                if x > 0 && char == self.characters[y][x - 1] {
                    sides -= 1;
                    if self.possibilities.contains(&(y, x - 1)) {
                        frontier.push_back((y, x - 1));
                        self.possibilities.retain(|&r| r != (y, x - 1));
                    }
                }
                parameter += sides;
            }

            total += parameter * area;
        }

        println!("Puzzle One: {:?}", total);
    }

    fn two(&mut self) {
        let mut total = 0;
        while self.possibilities.len() > 0 {
            let (y, x) = self.possibilities.remove(0);
            let char = self.characters[y][x];

            if !char.eq(&'.') {
                let mut parameter = 0;
                let mut area = 0;

                let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
                frontier.push_back((y, x));
                let mut y = 0;
                let mut x = 0;
                while frontier.len() > 0 {
                    (y, x) = frontier.pop_front().unwrap();
                    area += 1;
                    let mut sides = 4;
                    let mut side_vec: Vec<usize> = Vec::new(); // 0 = up, 1 = right, 2 = down, 3 = up
                    if y < self.height - 1 && char == self.characters[y + 1][x] {
                        side_vec.push(2);
                        sides -= 1;
                        if self.possibilities.contains(&(y + 1, x)) {
                            frontier.push_back((y + 1, x));
                            self.possibilities.retain(|&r| r != (y + 1, x));
                        }
                    }
                    if y > 0 && char == self.characters[y - 1][x] {
                        side_vec.push(0);
                        sides -= 1;
                        if self.possibilities.contains(&(y - 1, x)) {
                            frontier.push_back((y - 1, x));
                            self.possibilities.retain(|&r| r != (y - 1, x));
                        }
                    }
                    if x < self.width - 1 && char == self.characters[y][x + 1] {
                        side_vec.push(1);
                        sides -= 1;
                        if self.possibilities.contains(&(y, x + 1)) {
                            frontier.push_back((y, x + 1));
                            self.possibilities.retain(|&r| r != (y, x + 1));
                        }
                    }
                    if x > 0 && char == self.characters[y][x - 1] {
                        side_vec.push(3);
                        sides -= 1;
                        if self.possibilities.contains(&(y, x - 1)) {
                            frontier.push_back((y, x - 1));
                            self.possibilities.retain(|&r| r != (y, x - 1));
                        }
                    }
                    match sides {
                        4 => {
                            parameter += 4;
                        }
                        3 => {
                            parameter += 2;
                        }
                        2 => {
                            if ((side_vec[0] as isize - side_vec[1] as isize).abs() % 4 == 1)
                                || ((side_vec[0] as isize - side_vec[1] as isize).abs() % 4 == 3)
                            {
                                parameter += 1;
                                let opposites = opposite_corner(side_vec[0], side_vec[1]);
                                if (y as isize + opposites.1) >= 0
                                    && (y as isize + opposites.1) < self.height as isize
                                    && (x as isize + opposites.0) >= 0
                                    && (x as isize + opposites.0) < self.width as isize
                                    && self.characters[(y as isize + opposites.1) as usize]
                                        [(x as isize + opposites.0) as usize]
                                        != char
                                {
                                    parameter += 1;
                                }
                            } else {
                            }
                        }
                        1 => {
                            let check = tri_side_corners(
                                (0..4).find(|&side| !side_vec.contains(&side)).unwrap(),
                            );
                            if self.characters[(y as isize + check.0 .1) as usize]
                                [(x as isize + check.0 .0) as usize]
                                != char
                            {
                                parameter += 1;
                            }
                            if self.characters[(y as isize + check.1 .1) as usize]
                                [(x as isize + check.1 .0) as usize]
                                != char
                            {
                                parameter += 1;
                            }
                        }
                        _ => {
                            for dx in DELTAS {
                                for dy in DELTAS {
                                    if self.characters[(y as isize + dy) as usize]
                                        [(x as isize + dx) as usize]
                                        != char
                                    {
                                        parameter += 1;
                                    }
                                }
                            }
                        }
                    }
                }
                if parameter % 2 != 0 {
                    println!("P: {} | A: {} | T: {}, CHAR: {}, X: {} | Y: {}", parameter, area, parameter * area, char, x, y);
                }
                total += parameter * area;
            }
        }

        println!("Puzzle Two: {:?}", total);
    }
}

fn opposite_corner(side1: usize, side2: usize) -> (isize, isize) {
    match (side1, side2) {
        (0, 1) | (1, 0) => (1, -1),
        (1, 2) | (2, 1) => (1, 1),
        (2, 3) | (3, 2) => (-1, 1),
        (3, 0) | (0, 3) => (-1, -1),
        _ => (0, 0),
    }
}

fn tri_side_corners(side: usize) -> ((isize, isize), (isize, isize)) {
    match side {
        0 => ((1, 1), (-1, 1)),
        1 => ((-1, -1), (-1, 1)),
        2 => ((1, -1), (-1, -1)),
        3 => ((1, -1), (1, 1)),
        _ => ((0, 0), (0, 0)),
    }
}
