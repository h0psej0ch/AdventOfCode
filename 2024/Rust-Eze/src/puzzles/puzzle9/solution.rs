pub fn solve() {
    let file_path = "src/puzzles/puzzle9/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let mut values = Vec::new();
    contents.trim().chars().enumerate().for_each(|(i, char)| {
        let value: isize;
        if i % 2 == 0 {
            value = (i / 2) as isize;
        } else {
            value = -1;
        }
        for _j in 0..char.to_digit(10).unwrap() {
            values.push(value);
        }
    });

    let mut index = 0;
    while index < values.len() {
        if values[index] == -1 {
            let mut value = values.pop().unwrap();
            while value == -1 {
                value = values.pop().unwrap();
            }
            values[index] = value;
        }
        index += 1;
    }
    println!(
        "Puzzle One: {}",
        values
            .iter()
            .enumerate()
            .map(|(mul, id)| mul * *id as usize)
            .sum::<usize>()
    )
}

fn two(contents: &String) {
    let mut values = Vec::new();
    contents.trim().chars().enumerate().for_each(|(i, char)| {
        let value: isize;
        if i % 2 == 0 {
            value = (i / 2) as isize;
        } else {
            value = -1;
        }
        if char != '0' {
            values.push((value, char.to_digit(10).unwrap()))
        }
    });

    let mut index = values.len() - 1;
    while index > 0 {
        if values[index].0 != -1 {
            for j in 0..index {
                if values[j].0 == -1 {
                    if values[j].1 == values[index].1 {
                        let replace = values.remove(index);
                        values.insert(index, (-1, replace.1));
                        values[j] = replace;
                        break;
                    } else if values[j].1 > values[index].1 {
                        let replace = values.remove(index);
                        values.insert(index, (-1, replace.1));
                        values[j] = (values[j].0, values[j].1 - replace.1);
                        values.insert(j, replace);
                        index += 1;
                        break;
                    }
                }
            }
        }
        index -= 1;
    }

    let mut final_list = Vec::new();
    for tuple in values {
        for _i in 0..tuple.1 {
            final_list.push(tuple.0);
        }
    }

    println!(
        "Puzzle One: {}",
        final_list
            .iter()
            .enumerate()
            .filter(|tuple| *tuple.1 != -1)
            .map(|(mul, id)| mul * *id as usize)
            .sum::<usize>()
    )
}
