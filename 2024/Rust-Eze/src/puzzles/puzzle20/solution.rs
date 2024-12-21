use std::collections::VecDeque;

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn solve() {
    let file_path = "src/puzzles/puzzle20/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    let mut solver = Solver::new(contents);
    solver.path();

    solver.one();
    solver.two();
}

struct Solver {
    maze: Vec<Vec<char>>,
    costs: Vec<(isize, isize)>,
    sy: isize,
    sx: isize,
}

impl Solver {
    fn new(contents: String) -> Self {
        let maze = contents
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let (sy, sx): (isize, isize) = maze
            .iter()
            .enumerate()
            .filter(|(_y, line)| line.contains(&'S'))
            .map(|(y, line)| {
                (
                    y as isize,
                    line.iter().position(|t| *t == 'S').unwrap() as isize,
                )
            })
            .collect::<Vec<_>>()[0];

        let costs = Vec::new();

        Self {
            maze,
            costs,
            sy,
            sx,
        }
    }

    fn one(&mut self) {

        let mut counter = 0;

        let total = self.costs.len();
        self.costs.iter().enumerate().filter(|(index, _)| index <= &(total - 100)).for_each(|(index, (y,x))| {
            for cheat_size in index + 102..total {
                let (ny, nx) = self.costs[cheat_size];
                if y.abs_diff(ny) + x.abs_diff(nx) == 2 {
                    counter += 1;
                }
            }
        });

        println!("Puzzle One: {}", counter);
    }

    fn path(&mut self) {
        let mut queue: VecDeque<((isize, isize), isize)> = VecDeque::new();
        queue.push_back(((self.sy, self.sx), 0));

        while let Some(current) = queue.pop_front() {
            self.costs.push(current.0);
            for (dy, dx) in DIRECTIONS {
                let (ny, nx) = (current.0.0 + dy, current.0.1 + dx);
                if !self.costs.contains(&(ny, nx)) && ny > 0 && nx > 0 && ny < self.maze.len() as isize - 1 && nx < self.maze[0].len() as isize - 1 {
                    if self.maze[ny as usize][nx as usize] != '#' {
                        queue.push_back(((ny, nx), current.1 + 1))
                    }
                }
            }
        }
    }

    fn two(&mut self) {
        let mut counter = 0;

        let total = self.costs.len();
        self.costs.iter().enumerate().filter(|(index, _)| index <= &(total - 100)).for_each(|(index, (y,x))| {
            for cheat_size in index + 100..total {
                let (ny, nx) = self.costs[cheat_size];
                if y.abs_diff(ny) + x.abs_diff(nx) <= 20 && cheat_size >= index + y.abs_diff(ny) + x.abs_diff(nx) + 100 {
                    counter += 1;
                }
            }
        });

        println!("Puzzle Two: {}", counter);
    }
}
