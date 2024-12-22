use std::collections::HashMap;
use itertools::Itertools;

pub fn solve() {
    let file_path = "src/puzzles/puzzle22/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let count = contents
        .lines()
        .map(|string| string.parse::<usize>().unwrap())
        .map(|mut num| {
            for _ in 0..2000 {
                num = num ^ (num << 6);
                num = num & 16777215;
                num = num ^ (num >> 5);
                num = num & 16777215;
                num = num ^ (num << 11);
                num = num & 16777215;
            }
            num
        }).sum::<usize>();

    println!("Puzzle One: {}", count)
}

fn two(contents: &String) {
    let mut big_map: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
    contents
        .lines()
        .map(|string| string.parse::<usize>().unwrap())
        .for_each(|mut num| {
            let mut numbers = Vec::new();
            let mut instructions: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
            for _ in 0..2000 {
                numbers.push(num % 10);
                num = num ^ (num << 6);
                num = num & 16777215;
                num = num ^ (num >> 5);
                num = num & 16777215;
                num = num ^ (num << 11);
                num = num & 16777215;
            }
            let changes = numbers.windows(5)
                .map(|vec| vec.iter()
                    .tuple_windows()
                    .map(|(&x, &y)| y as isize - x as isize)
                    .collect_tuple::<(isize, isize, isize, isize)>()
                    .unwrap()
                )
                .collect::<Vec<_>>();
            for i in 0..changes.len() {
                if !instructions.contains_key(&changes[i]) {
                    instructions.insert(changes[i], numbers[i + 4] as isize);
                }
            }
            for (key, value) in instructions {
                *big_map.entry(key).or_insert(0) += value;
            }
        });
    println!("Puzzle Two: {:?}", big_map.values().max().unwrap())
}
