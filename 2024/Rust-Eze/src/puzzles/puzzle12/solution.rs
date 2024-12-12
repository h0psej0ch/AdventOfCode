use std::collections::VecDeque;

pub fn solve() {
    
    let file_path = "src/puzzles/puzzle12/input.txt";

    let contents = std::fs::read_to_string(file_path)
        .expect("No input file");

    let mut solver = Solver::new(contents);
    solver.one();
    solver.two();

}

struct Solver {
    characters: Vec<Vec<char>>,
    possibilities: Vec<(usize, usize)>,
    islands: Vec<Vec<(usize, usize)>>,
    height: usize,
    width: usize

}

impl Solver {

    fn new(contents: String) -> Self {

        let characters = contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

        let mut possibilities: Vec<(usize, usize)> = Vec::new();

        let height = characters.len();
        let width = characters[0].len();

        for y in 0..height {
            for x in 0..width {
                possibilities.push((y, x));
            }
        }

        let islands: Vec<Vec<(usize, usize)>> = Vec::new();

        Solver {
            characters,
            possibilities,
            islands,
            height,
            width
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
                if y < self.height - 1 && char == self.characters[y+1][x] {
                    sides -= 1;
                    if self.possibilities.contains(&(y+1, x)) {
                        frontier.push_back((y + 1, x));
                        self.possibilities.retain(|&r| r != (y + 1, x));
                    }
                }
                if y > 0 && char == self.characters[y-1][x] {
                    sides -= 1;
                    if self.possibilities.contains(&(y-1, x)) {
                        frontier.push_back((y - 1, x));
                        self.possibilities.retain(|&r| r != (y - 1, x));
                    }
                }
                if x < self.width - 1 && char == self.characters[y][x+1] {
                    sides -= 1;
                    if self.possibilities.contains(&(y, x+1)) {
                        frontier.push_back((y, x + 1));
                        self.possibilities.retain(|&r| r != (y, x + 1));
                    }
                }
                if x > 0 && char == self.characters[y][x-1] {
                    sides -= 1;
                    if self.possibilities.contains(&(y, x-1)) {
                        frontier.push_back((y, x - 1));
                        self.possibilities.retain(|&r| r != (y, x - 1));
                    }
                }
                parameter += sides;
            }

            total += parameter * area;
        }

        println!("Puzzle Two: {:?}", total);
    }

    fn two(&mut self) {

    }
}