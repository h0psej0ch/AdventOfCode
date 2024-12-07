pub fn solve() {
    let file_path = "src/puzzles/puzzle7/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let sum = contents
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(result, numbers)| {
            (
                result.parse::<i64>().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|string| string.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(results, numbers)| check_valid(*results, (*numbers).to_owned()))
        .map(|(a, b)| a)
        .sum::<i64>();

    println!("Puzzle One: {}", sum);
}

fn check_valid(result: i64, mut numbers: Vec<i64>) -> bool {
    if result == 0 {
        true
    } else if result < 0 || numbers.len() == 0 {
        false
    } else {
        let next = numbers.pop().unwrap();
        return if result % next == 0 {
            check_valid(result / next, numbers.clone()) || check_valid(result - next, numbers)
        } else {
            check_valid(result - next, numbers)
        };
    }
}

fn check_valid_concat(result: i64, mut numbers: Vec<i64>) -> bool {
    if result == 0 && numbers.len() == 0 {
        true
    } else if result < 0 || numbers.len() == 0 {
        false
    } else {
        let next = numbers.pop().unwrap();
        let string_next = next.to_string();
        let string_result = result.to_string();
        return if string_result.ends_with(&string_next) {
            let string_result_new = string_result.strip_suffix(&string_next).unwrap();
            if string_result_new.len() == 0 {
                return true;
            }
            if result % next == 0 {
                check_valid_concat(string_result_new.parse::<i64>().unwrap(), numbers.clone()) ||
                    check_valid_concat(result / next, numbers.clone())
                    || check_valid_concat(result - next, numbers)
            } else {
                check_valid_concat(string_result_new.parse::<i64>().unwrap(), numbers.clone()) ||
                check_valid_concat(result - next, numbers)
            }
        } else {
            if result % next == 0 {
                check_valid_concat(result / next, numbers.clone())
                    || check_valid_concat(result - next, numbers)
            } else {
                check_valid_concat(result - next, numbers)
            }
        }
    }
}

fn two(contents: &String) {
    let sum = contents
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(result, numbers)| {
            (
                result.parse::<i64>().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|string| string.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|(results, numbers)| check_valid_concat(*results, (*numbers).to_owned()))
        .map(|(a, b)| a)
        .sum::<i64>();

    println!("Puzzle Two: {}", sum);
}
