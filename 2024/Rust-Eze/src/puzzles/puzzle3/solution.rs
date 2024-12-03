pub fn solve() {
    
    let file_path = "src/puzzles/puzzle3/input.txt";

    let contents = std::fs::read_to_string(file_path)
        .expect("No input file");

    one(&contents);
    two(&contents);

}

fn one(contents: &String) {

    let mut result = 0;

    for line in contents.lines() {

        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i + 7 < chars.len() {
            if chars[i] == 'm' && chars[i+1] == 'u' && chars[i+2] == 'l' && chars[i+3] == '(' {
                let mut j = i + 4;

                let mut bracket = 0;
                let mut comma = 0;

                while j < chars.len() {
                    if chars[j] == ')' {
                        bracket = j;
                        break;
                    }
                    else if chars[j] == ',' {
                        comma = j;
                    }
                    else if !(chars[j].is_digit(10)) {
                        break;
                    }
                    j += 1;
                }

                if comma != 0 && bracket != 0 {

                    println!("{}", &line[i..=bracket]);

                    let first = &line[i + 4..comma];
                    let second = &line[comma + 1..bracket];

                    let first = first.parse::<usize>().unwrap();
                    let second = second.parse::<usize>().unwrap();

                    result += first * second;

                    i = bracket;
                }
                else {
                    i += 3
                }
            }
            i += 1;
        }
    }

    println!("Puzzle One: {result}")
}

fn two(contents: &String) {

    let mut result = 0;
    let mut active = true;

    for line in contents.lines() {

        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        while i + 7 < chars.len() {
            if active && chars[i] == 'm' && chars[i+1] == 'u' && chars[i+2] == 'l' && chars[i+3] == '(' {
                let mut j = i + 4;

                let mut bracket = 0;
                let mut comma = 0;

                while j < chars.len() {
                    if chars[j] == ')' {
                        bracket = j;
                        break;
                    }
                    else if chars[j] == ',' {
                        comma = j;
                    }
                    else if !(chars[j].is_digit(10)) {
                        break;
                    }
                    j += 1;
                }

                if comma != 0 && bracket != 0 {

                    println!("{}", &line[i..=bracket]);

                    let first = &line[i + 4..comma];
                    let second = &line[comma + 1..bracket];

                    let first = first.parse::<usize>().unwrap();
                    let second = second.parse::<usize>().unwrap();

                    result += first * second;

                    i = bracket;
                }
                else {
                    i += 3
                }
            }
            else if &line[i..=i+3] == "do()" {
                active = true;
            }
            else if &line[i..=i+6] == "don't()" {
                active = false;
            }

            i += 1;
        }
    }

    println!("Puzzle Two: {result}")
}