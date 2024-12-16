use std::collections::HashMap;

pub fn solve() {
    let file_path = "src/puzzles/puzzle16/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    let mut solver = Solver::new(contents);

    solver.one();
    solver.two();
}

struct Solver {
    maze: Vec<Vec<char>>,
}

impl Solver {
    fn new(contents: String) -> Self {
        let maze = contents
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Self {
            maze,
        }
    }

    fn one(&mut self) {
        let (sy, sx): (isize, isize) = self
            .maze
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

        let (ey, ex): (isize, isize) = self
            .maze
            .iter()
            .enumerate()
            .filter(|(_y, line)| line.contains(&'E'))
            .map(|(y, line)| {
                (
                    y as isize,
                    line.iter().position(|t| *t == 'E').unwrap() as isize,
                )
            })
            .collect::<Vec<_>>()[0];

        let mut unvisited: HashMap<((isize, isize), (isize, isize)), isize> = HashMap::new();
        let mut visited: HashMap<((isize, isize), (isize, isize)), isize> = HashMap::new();

        unvisited.insert(((sy, sx), (0, 1)), 0);

        while !unvisited.is_empty() {
            let current = unvisited
                .iter()
                .min_by(|(_a, b), (_x, y)| b.cmp(y))
                .map(|(k, v)| (k.clone(), *v))
                .unwrap();
            unvisited.remove(&current.0);
            visited.insert(current.0.clone(), current.1);

            if current.0.0.0 == ey && current.0.0.1 == ex {
                break;
            }

            if !visited.contains_key(&((current.0).0, ((current.0).1.1, (current.0).1.0))) {
                unvisited.insert(((current.0).0, ((current.0).1.1, (current.0).1.0)), current.1 + 1000);
            }
            if !visited.contains_key(&((current.0).0, (-(current.0).1.1, -(current.0).1.0))) {
                unvisited.insert(((current.0).0, (-(current.0).1.1, -(current.0).1.0)), current.1 + 1000);
            }

            let (ny, nx) = (current.0.0.0 + current.0.1.0, current.0.0.1 + current.0.1.1);
            if ny >= 0
                && nx >= 0
                && ny < self.maze.len() as isize
                && nx < self.maze[0].len() as isize
                && self.maze[ny as usize][nx as usize] != '#'
                && !visited.contains_key(&((ny, nx), current.0.1))
            {
                unvisited.insert(((ny, nx), current.0.1), current.1 + 1);
            }
        }

        let answer = visited.iter().filter(|(((y, x), _b), _c)| *x == ex && *y == ey).min_by(|(_a, b), (_x, y,)| b.cmp(y)).unwrap();

        println!("Puzzle One: {}", answer.1)
    }

    fn two(&mut self) {}
}
