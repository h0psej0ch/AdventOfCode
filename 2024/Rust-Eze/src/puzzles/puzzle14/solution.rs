use itertools::Itertools;
use regex::Regex;

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

pub fn solve() {
    let file_path = "src/puzzles/puzzle14/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let re = Regex::new(r"p[+=](-?\d+),(-?\d+) v[+=](-?\d+),(-?\d+)").unwrap();
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut fourth = 0;
    contents
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|regex| capture_to_numbers(&regex))
        .map(|((x, y), (dx, dy))| {
            (
                ((x + 100 * dx) % WIDTH + WIDTH) % WIDTH,
                ((y + 100 * dy) % HEIGHT + HEIGHT) % HEIGHT,
            )
        })
        .for_each(|(x, y)| {
            if y < HEIGHT / 2 {
                if x < WIDTH / 2 {
                    first += 1;
                } else if x > WIDTH / 2 {
                    second += 1;
                }
            } else if y > HEIGHT / 2 {
                if x < WIDTH / 2 {
                    third += 1;
                } else if x > WIDTH / 2 {
                    fourth += 1;
                }
            }
        });

    println!("Puzzle One: {:?}", first * second * third * fourth)
}

fn capture_to_numbers(capture: &regex::Captures) -> ((isize, isize), (isize, isize)) {
    (
        (
            capture.get(1).unwrap().as_str().parse::<isize>().unwrap(),
            capture.get(2).unwrap().as_str().parse::<isize>().unwrap(),
        ),
        (
            capture.get(3).unwrap().as_str().parse::<isize>().unwrap(),
            capture.get(4).unwrap().as_str().parse::<isize>().unwrap(),
        ),
    )
}

fn two(contents: &String) {
    let re = Regex::new(r"p[+=](-?\d+),(-?\d+) v[+=](-?\d+),(-?\d+)").unwrap();
    let mut positions: Vec<_> = contents
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|regex| capture_to_numbers(&regex))
        .collect();

    let mut i = 0;
    loop {
        i += 1;
        positions = positions
            .iter()
            .map(|((x, y), (dx, dy))| {
                (
                    (((x + dx) % WIDTH + WIDTH) % WIDTH, ((y + dy) % HEIGHT + HEIGHT) % HEIGHT),
                    (*dx, *dy)
                )
            })
            .collect::<Vec<_>>();
        if positions.iter().unique_by(|((x, y), (_dx, _dy))| (x, y)).collect::<Vec<_>>().len() == 500 {
            println!("Puzzle Two: {}", i);
            // tree_printer(positions.clone());
            break;
        }
    }
}

#[allow(dead_code)]
fn tree_printer(positions: Vec<((isize, isize), (isize, isize))>) {
    let points: Vec<_> = positions.iter().unique_by(|((x, y), (_dx, _dy))| (x, y)).map(|(a, _b)| a).collect();
    for y in 0..HEIGHT {
        for x in 0..HEIGHT {
            if points.contains(&&(x, y)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}