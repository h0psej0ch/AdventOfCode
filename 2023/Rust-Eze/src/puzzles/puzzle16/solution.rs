use std::collections::HashSet;

pub fn solve() {
    let file_path = "src/puzzles/puzzle16/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    let mut map = Map::new(&contents);

    map.one();
    map.two();
}

struct Map {
    map: Vec<Vec<char>>,
    energized: HashSet<(i32, i32, (i32, i32))>,
    height: i32,
    length: i32,
}

impl Map {
    fn new(contents: &str) -> Self {
        let map: Vec<Vec<char>> = contents
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let height = map.len();
        let length = map[height-1].len() as i32;

        Self {
            map,
            energized: HashSet::new(),
            height: height as i32,
            length,
        }
    }

    fn one(&mut self) {

        println!("{}", self.get_energized((-1,0), (1, 0)));

        // for y in 0..self.height {
        //     for x in 0..self.length {
        //         if result.contains(&(y, x,)) {
        //             print!("#");
        //         } else {
        //             print!("{}", self.map[y as usize][x as usize]);
        //         }
        //     }
        //     println!();
        // }
    }

    fn two(&mut self) {
        let mut highest: usize = 0;
        for i in 0..=self.length + 3 {
            let new = self.get_energized((i, -1), (0, 1));
            if new > highest {
                highest = new;
            }
            let new = self.get_energized((i, self.height), (0, -1));
            if new > highest {
                highest = new;
            }     
        }
        for i in 0..=self.height + 3 {
            let new = self.get_energized((-1, i), (1, 0));
            if new > highest {
                highest = new;
            }
            let new = self.get_energized((self.length, i), (-1, 0));
            if new > highest {
                highest = new;
            } 
        }

        println!("{}", highest)

    }
    
    fn get_energized(&mut self, (x, y): (i32, i32), direction: (i32, i32)) -> usize {
        self.energized.clear();
        self.traverse((x, y), direction);

        let mut result: HashSet<(i32, i32)> = HashSet::new();
        for coordinate in &self.energized {
            result.insert((coordinate.0, coordinate.1));
        };
        result.len()
    }

    fn traverse(&mut self, (x, y): (i32, i32), direction: (i32, i32)) {
        let x: i32 = x + direction.0;
        let y: i32 = y + direction.1;
        if x >= 0
            && y >= 0
            && y < self.height
            && x < self.length
            && !self.energized.contains(&(y, x, direction))
        {
            self.energized.insert((y, x, direction));
            match self.map[y as usize][x as usize] {
                '/' => {
                    self.traverse((x, y), self.right_mirror(direction))
                },
                '\\' => {
                    self.traverse((x, y), self.left_mirror(direction))
                },
                '|' => {
                    if direction == (0, 1) || direction == (0, -1) {
                        self.traverse((x, y), direction);
                    } else {
                        self.traverse((x, y), (0, 1));
                        self.traverse((x, y), (0, -1));
                    }
                },
                '-' => {
                    if direction == (1, 0) || direction == (-1, 0) {
                        self.traverse((x, y), direction);
                    } else {
                        self.traverse((x, y), (1, 0));
                        self.traverse((x, y), (-1, 0));
                    }
                }
                _ => {
                   self.traverse((x, y), direction); 
                }
            }
        }
    }

    fn right_mirror(&self, direction: (i32, i32)) -> (i32, i32) {
        match direction {
            (0, 1) => return (-1, 0),
            (0, -1) => return (1, 0),
            (1, 0) => return (0, -1),
            (-1, 0) => return (0, 1),
            _ => return direction
        }
    }

    fn left_mirror(&self, direction: (i32, i32)) -> (i32, i32) {
        match direction {
            (0, 1) => return (1, 0),
            (0, -1) => return (-1, 0),
            (1, 0) => return (0, 1),
            (-1, 0) => return (0, -1),
            _ => return direction
        }
    }

    // fn absolute_direction(&self, direction: (i32, i32)) -> (i32, i32) {
    //     match direction {
    //         (0, -1) => return (0, 1),
    //         (-1, 0) => return (1, 0),
    //         _ => return direction
    //     }
    // }
}
