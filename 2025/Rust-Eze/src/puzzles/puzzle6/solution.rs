pub fn solve() {
    let file_path = "src/puzzles/puzzle6/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");
    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let numbers = transpose(
        contents
            .lines()
            .take(contents.lines().count() - 1)
            .map(|line| {
                line.split_whitespace()
                    .map(|c| c.parse::<u128>().unwrap())
                    .collect()
            })
            .collect(),
    );
    let operators: Vec<char> = contents
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|c| c.chars().collect::<Vec<char>>()[0])
        .collect();

    println!(
        "Puzzle 6.1: {}",
        numbers
            .iter()
            .enumerate()
            .map(|(n, num)| match operators[n] {
                '+' => num.iter().copied().reduce(|x, y| x + y).unwrap(),
                '*' => num.iter().copied().reduce(|x, y| x * y).unwrap(),
                _ => 0_u128,
            })
            .sum::<u128>()
    );
}

fn two(contents: &String) {
    let lines = contents
        .lines()
        .map(|c| c.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut operator = true; // true = +, false = *
    let mut total: u128 = 0;
    let mut current: u128 = 0;
    for i in 0..lines[0].len() {
        let mut number: u128 = 0;
        for j in 0..lines.len() - 1 {
            if lines[j][i] != ' ' {
                number = number * 10 + lines[j][i].to_digit(10).unwrap() as u128;
            }
        }
        if number == 0 {
            total += current;
            current = 0;
        } else {
            if lines[lines.len() - 1][i] != ' ' {
                operator = lines[lines.len() - 1][i] == '+';
            }
            if operator {
                current += number;
            } else {
                current = if current == 0 {
                    number
                } else {
                    current * number
                };
            }
        }
    }
    total += current;
    println!("Puzzle 6.2: {}", total);
}

fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    for _ in 0..v[0].len() {
        result.push(Vec::<T>::new());
    }
    for x in 0..v.len() {
        for y in 0..v[0].len() {
            result[y].push(v[x][y].clone());
        }
    }
    result
}
