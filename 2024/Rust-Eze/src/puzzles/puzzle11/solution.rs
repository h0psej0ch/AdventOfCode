use std::collections::HashMap;

pub fn solve() {
    let file_path = "src/puzzles/puzzle11/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let mut stones = contents
        .lines()
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .split_whitespace()
        .map(|string| string.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for _ in 0..25 {
        let mut new_stones = Vec::new();
        stones
            .iter()
            .for_each(|&stone| {
                if stone == 0 {
                    new_stones.push(1);
                } else if stone.ilog10() % 2 == 1 {
                    let split = (stone.ilog10() + 1) / 2;
                    new_stones.push(stone / (10_usize.pow(split)));
                    new_stones.push(stone % (10_usize.pow(split)));
                } else {
                    new_stones.push(stone * 2024)
                }
            });
        stones = new_stones;
    }

    println!("Puzzle One: {}", stones.len())
}

fn two(contents: &String) {
    let mut stone_count: HashMap<usize, usize> = HashMap::new();
    contents
        .lines()
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .split_whitespace()
        .for_each(|string| {
            *stone_count
                .entry(string.parse::<usize>().unwrap())
                .or_insert(0) += 1
        });

    for _ in 0..75 {
        let mut new_stones: HashMap<usize, usize> = HashMap::new();
        stone_count
            .iter()
            .for_each(|(&stone, &count)| {
                if stone == 0 {
                    *new_stones.entry(1).or_insert(0) += count;
                } else if stone.ilog10() % 2 == 1 {
                    let split = (stone.ilog10() + 1) / 2;
                    *new_stones.entry(stone % (10_usize.pow(split))).or_insert(0) += count;
                    *new_stones.entry(stone / (10_usize.pow(split))).or_insert(0) += count;
                } else {
                    *new_stones.entry(stone * 2024).or_insert(0) += count;
                }
            });
        stone_count = new_stones;
    }

    println!("Puzzle Two: {}", stone_count.iter().map(|(&_stone, &count)| count).sum::<usize>())
}
