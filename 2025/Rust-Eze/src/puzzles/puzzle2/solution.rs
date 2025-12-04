use std::collections::HashSet;

pub fn solve() {
    let file_path = "src/puzzles/puzzle2/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &str) {
    let result: u128 = contents
        .split(',')
        .map(|ran| ran.split_once('-').unwrap())
        .map(|(val1, val2)| {
            (
                val1.trim().parse::<u128>().unwrap(),
                val2.trim().parse::<u128>().unwrap(),
            )
        })
        .map(|(val1, val2)| {
            (val1..=val2)
                .filter(|val| {
                    let logged = 10_u128.pow((*val as f64).log(10.0).ceil() as u32 / 2);
                    *val == (val % logged) * (logged + 1)
                })
                .sum::<u128>()
        })
        .sum();
    println!("Puzzle 2.1: {}", result);
}

fn two(contents: &str) {
    let result: u128 = contents
        .split(',')
        .map(|ran| ran.split_once('-').unwrap())
        .map(|(val1, val2)| {
            (
                val1.trim().parse::<u128>().unwrap(),
                val2.trim().parse::<u128>().unwrap(),
            )
        })
        .map(|(lower, upper)| generate_repetitions(lower, upper))
        .sum();
    println!("Puzzle 2.2: {}", result);
}

fn generate_repetitions(lower: u128, upper: u128) -> u128 {
    let low_len = (lower as f64).log10() as u32 + 1;
    let high_len = (upper as f64).log10() as u32 + 1;

    let mut hits: HashSet<u128> = HashSet::new();

    for len in low_len..=high_len {
        (1..=len / 2).filter(|n| len % n == 0).for_each(|n| {
            (10_u128.pow(n - 1)..10_u128.pow(n))
                .map(|m| {
                    (0..len / n)
                        .map(|x| 10_u128.pow(n).pow(x) * m)
                        .sum::<u128>()
                })
                .filter(|&num| num >= lower && num <= upper)
                .for_each(|num| {
                    hits.insert(num);
                });
        });
    }
    hits.into_iter().sum::<u128>()
}
