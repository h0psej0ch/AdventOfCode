use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn solve() {
    let file_path = "src/puzzles/puzzle18/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let mut obstacles: HashSet<(isize, isize)> = HashSet::new();
    contents
        .lines()
        .enumerate()
        .filter(|(num, _line)| *num < 1024)
        .map(|(_num, line)| {
            line.split(",")
                .map(|string| string.parse::<isize>().unwrap())
                .collect_tuple::<(isize, isize)>()
                .unwrap()
        })
        .for_each(|tuple| {
            obstacles.insert(tuple);
        });

    println!("Puzzle One: {}", find_path(obstacles))
}

fn two(contents: &String) {
    let mut min = 0;
    let mut obstacles: HashSet<(isize, isize)> = HashSet::new();
    let bytes = contents
        .lines()
        .enumerate().collect::<Vec<_>>();
    for (num, line) in bytes {

        obstacles.insert(line.split(",")
            .map(|string| string.parse::<isize>().unwrap())
            .collect_tuple::<(isize, isize)>()
            .unwrap());

        if num <= 1024 {
            continue;
        }

        if find_path(obstacles.clone()) == -1 && min == 0 {
            println!("Puzzle Two: {}", line);
            break;
        }
    }
}

fn find_path(obstacles: HashSet<(isize, isize)>) -> isize {
    let mut frontier: HashMap<(isize, isize), isize> = HashMap::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    frontier.insert((0, 0), 0);

    while !frontier.is_empty() {
        let (&(x, y), &score) = frontier.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
        frontier.remove(&(x, y));

        if x == 70 && y == 70 {
            return score
        }

        visited.insert((x, y));

        for (dx, dy) in DIRECTIONS {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0
                && ny >= 0
                && nx <= 70
                && ny <= 70
                && !obstacles.contains(&(nx, ny))
                && !visited.contains(&(nx, ny))
                && !frontier.contains_key(&(nx, ny))
            {
                frontier.insert((nx, ny), score + 1);
            }
        }
    }
    -1
}
