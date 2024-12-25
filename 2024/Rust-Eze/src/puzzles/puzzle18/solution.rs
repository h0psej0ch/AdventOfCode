use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

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

    println!("Puzzle One: {}", find_cost(obstacles))
}

fn two(contents: &String) {
    let mut path = HashSet::new();
    let mut obstacles: HashSet<(isize, isize)> = HashSet::new();
    let bytes = contents
        .lines()
        .enumerate().collect::<Vec<_>>();
    for (num, line) in bytes {

        let new_byte = line.split(",")
            .map(|string| string.parse::<isize>().unwrap())
            .collect_tuple::<(isize, isize)>()
            .unwrap();

        obstacles.insert(new_byte);

        if num <= 1024 {
            continue;
        }

        if path.contains(&new_byte) || path.is_empty() {
            if let Some(new_path) = find_path(obstacles.clone()) {
                path = new_path
            } else {
                println!("Puzzle Two: {}", line);
                break;
            }
        }
    }
}

fn find_cost(obstacles: HashSet<(isize, isize)>) -> isize {
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

fn find_path(obstacles: HashSet<(isize, isize)>) -> Option<HashSet<(isize, isize)>> {
    let mut frontier: VecDeque<(isize, isize)> = VecDeque::new();
    let mut came_from: HashMap<(isize, isize), Option<(isize, isize)>> = HashMap::new();
    let mut cost_so_far: HashMap<(isize, isize), isize> = HashMap::new();

    frontier.push_back((0, 0));
    came_from.insert((0, 0), None);
    cost_so_far.insert((0, 0), 0);

    while let Some((x, y)) = frontier.pop_front() {

        if x == 70 && y == 70 {
            let mut path = HashSet::new();
            let mut current = (70, 70);
            while let Some(&Some(prev)) = came_from.get(&current) {
                path.insert(current);
                current = prev;
            }
            return Some(path)
        }

        for (dx, dy) in DIRECTIONS {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0
                && ny >= 0
                && nx <= 70
                && ny <= 70
                && !obstacles.contains(&(nx, ny))
            {
                let new_cost = cost_so_far[&(x, y)] + 1;
                if !cost_so_far.contains_key(&(nx, ny)) || new_cost < cost_so_far[&(nx, ny)] {
                    frontier.push_back((nx, ny));
                    cost_so_far.insert((nx, ny), new_cost);
                    came_from.insert((nx, ny), Some((x, y)));
                }
            }
        }
    }
    None
}

