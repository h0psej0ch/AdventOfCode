pub fn solve() {
    
    let file_path = "input.txt";

    let contents = std::fs::read_to_string(file_path)
        .expect("No input file");

    puzzle(&contents);

}

pub fn replace_numbers(input: &str) -> String {
    let mut iterator: usize = 0;
    let mut result = String::new();
    while iterator < input.len() as usize {
        match input.chars().nth(iterator) {
            Some('o') => {
                if input.len() >= iterator + 3 && &input[iterator..iterator + 3] == "one" {
                    result.push('1');
                } else { result.push('o');}
            }
            Some('t') => {
                if input.len() >= iterator + 3 && &input[iterator..iterator + 3] == "two" {
                    result.push('2');
                }
                else if input.len() >= iterator + 5 && &input[iterator..iterator + 5] == "three" {
                    result.push('3');
                }
                else { result.push('t');}
            }
            Some('f') => {
                if input.len() >= iterator + 4 && &input[iterator..iterator + 4] == "four" {
                    result.push('4');
                }
                else if input.len() >= iterator + 4 && &input[iterator..iterator + 4] == "five" {
                    result.push('5');
                }
                else { result.push('f');}
            }
            Some('s') => {
                if input.len() >= iterator + 3 && &input[iterator..iterator + 3] == "six" {
                    result.push('6');
                }
                else if input.len() >= iterator + 5 && &input[iterator..iterator + 5] == "seven" {
                    result.push('7');
                }
                else { result.push('s');}
            }
            Some('e') => {
                if input.len() >= iterator + 5 && &input[iterator..iterator + 5] == "eight" {
                    result.push('8');
                }
                else { result.push('e');}
            }
            Some('n') => {
                if input.len() >= iterator + 4 && &input[iterator..iterator + 4] == "nine" {
                    result.push('9');
                }
                else { result.push('n');}
            }

            Some(_) => { 
                result.push(input.chars().nth(iterator).unwrap());
            }
            None => { }
        }
        iterator += 1;
    }
    return result;
}

pub fn puzzle(input: &str) {
    println!("Puzzle from 1.rs");

    let mut sum: i32 = 0;
    let mut first: char = '0';
    let mut second: char = '0';

    for line in input.lines() {
        let line = replace_numbers(line); // remove for solution 1
        for character in line.chars() {
            match character.to_digit(10) {
                Some(_num) => {
                    first = character;
                    break;
                }
                None => continue,
            }
        }
        for character in line.chars().rev().collect::<Vec<char>>() {
            match character.to_digit(10) {
                Some(_num) => {
                    second = character;
                    break;
                }
                None => continue,
            }
        }
        sum += format!("{}{}", first, second).parse::<i32>().unwrap();
        println!("Sum: {} Using {}{} from {}", sum, first, second, line);
    }

}