use std::collections::{HashMap, VecDeque};

pub fn solve() {
    let file_path = "src/puzzles/puzzle22/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    let mut solver = Solver::new(contents);

    solver.fall_down();

    // solver.one();
    // solver.two();
}

struct Solver {
    blocks: Vec<((usize, usize, usize), (usize, usize, usize))>,
    fallen_blocks: HashMap<(usize), Vec<((usize, usize, usize), (usize, usize, usize))>>,
}

impl Solver {
    fn new(contents: String) -> Solver {
        let mut blocks: Vec<((usize, usize, usize), (usize, usize, usize))> = Vec::new();
        let mut fallen_blocks: HashMap<
            (usize),
            Vec<((usize, usize, usize), (usize, usize, usize))>,
        > = HashMap::new();

        // Separating all the inputs and storing them in putting them in the blocks vector
        for line in contents.lines() {
            let parts: Vec<_> = line.split("~").collect();
            let start: Vec<_> = parts[0].split(",").collect();
            let end: Vec<_> = parts[1].split(",").collect();
            blocks.push((
                (
                    start[0].parse().unwrap(),
                    start[1].parse().unwrap(),
                    start[2].parse().unwrap(),
                ),
                (
                    end[0].parse().unwrap(),
                    end[1].parse().unwrap(),
                    end[2].parse().unwrap(),
                ),
            ));
        }

        // for block in blocks.iter() {
        //     println!("{:?}", block);
        // }

        Solver {
            blocks,
            fallen_blocks,
        }
    }

    fn fall_down(&mut self) {
        // Sorting the blocks by the minimum z value
        self.blocks.sort_by(|a, b| {
            let a_min = a.0 .2.min(a.1 .2);
            let b_min = b.0 .2.min(b.1 .2);
            a_min.cmp(&b_min)
        });

        for block in self.blocks {
            if block.0 .2.min(block.1 .2) == 1 {
                self.fallen_blocks
                    .entry(1)
                    .or_insert(Vec::new())
                    .push(block);
            } else {
                let level = block.0 .2.min(block.1 .2);
                while level >= 1 {
                    if level == 1 {
                        self.fallen_blocks
                            .entry(1)
                            .or_insert(Vec::new())
                            .push(block);
                    } else {
                        if let Some(fallen_block_level) = self.fallen_blocks.get(&(level - 1)) {
                            let mut blocked = false;
                            for fallen_block in fallen_block_level {
                                //  Please refactor... It will not be able to handle fallen_blocks are taller than 1 :(
                            }
                            if blocked {
                                self.fallen_blocks
                                    .entry(level)
                                    .or_insert(Vec::new())
                                    .push(block);
                                break;
                            }
                        } else {
                            level -= 1;
                        }
                    }
                }
            }
        }

        for block in self.blocks.iter() {
            println!("{:?}", block);
        }
    }

    fn one() {}

    fn two() {}
}
