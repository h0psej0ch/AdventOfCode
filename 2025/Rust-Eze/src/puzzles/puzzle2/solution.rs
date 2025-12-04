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
        .map(|(val1, val2)| {
            (val1..=val2)
                .filter(|val| {
                    let dig_len = val.ilog10() + 1;
                    for i in 1..=dig_len / 2 {
                        if dig_len % i == 0 {
                            let pattern = val % 10_u128.pow(i);
                            if *val
                                == (0..dig_len / i)
                                    .map(|j| 10_u128.pow(i).pow(j) * pattern)
                                    .sum::<u128>()
                            {
                                return true;
                            }
                        }
                    }
                    false
                })
                .sum::<u128>()
        })
        .sum();
    println!("Puzzle 2.2: {}", result);
}
