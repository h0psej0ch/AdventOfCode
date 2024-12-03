pub fn solve() {
    let file_path = "src/puzzles/puzzle2/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let count = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|char| char.parse::<usize>().unwrap())
                .collect()
        })
        .filter(|line: &Vec<usize>| {
            line.windows(2)
                .all(|window| window[0] > window[1] && window[0].abs_diff(window[1]) <= 3)
                || line
                    .windows(2)
                    .all(|window| window[0] < window[1] && window[0].abs_diff(window[1]) <= 3)
        })
        .count();

    println!("Part One: {}", count)
}

fn two(contents: &String) {
    let mut counter: usize = 0;
    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut index = check_valid(&parts);
        if index > 0 {
            loop {
                let mut clone = parts.clone();
                clone.remove(index);
                if check_valid(&clone) == 0 {
                    counter += 1;
                    break;
                }
                if index == 0 {
                    break;
                }
                index -= 1;
            }
        } else {
            counter += 1;
        }
    }
    println!("Part Two: {}", counter);
}

fn check_valid(list: &Vec<&str>) -> usize {
    let mut operator = 0;
    let mut previous = 0;
    let mut passed = true;
    let mut index = 0;
    for character in list {
        let integer = character.parse::<usize>().unwrap();
        if operator == 0 {
            if !(previous == 0) {
                if previous == integer || integer.abs_diff(previous) > 3 {
                    passed = false;
                    break;
                } else if previous > integer {
                    operator = 1;
                } else {
                    operator = -1
                }
            }
        } else if (operator == 1 && previous <= integer)
            || (operator == -1 && previous >= integer)
            || integer.abs_diff(previous) > 3
        {
            passed = false;
            break;
        }
        previous = integer;
        index += 1;
    }
    if passed {
        0
    } else {
        index
    }
}
