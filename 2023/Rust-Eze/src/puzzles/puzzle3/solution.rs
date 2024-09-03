use std::collections::HashSet;
use std::collections::HashMap;

pub fn solve() {
    
    let file_path = "input.txt";

    let contents = std::fs::read_to_string(file_path)
        .expect("No input file");

    one(&contents);
    two(&contents);

}

fn one(contents: &String) {

    let mut symbols: HashSet<(usize, usize)> = HashSet::new();

    // numbers stored as (((beginX, endX), Y), Value)
    let mut digits: HashMap<((usize, usize), usize), usize> = HashMap::new(); 

    // Y coordinate in the grid
    let mut y: usize = 0;

    // keep track of the numbers
    let mut last = false;

    for line in contents.lines() {
        let mut x: usize = 0;
        let mut begin: usize = 0;
        for character in line.chars() {
            if character.is_digit(10) {
                if !last {
                    last = true;
                    begin = x;
                }
            }
            else {
                if character != '.' {
                    symbols.insert((x, y));
                }
                if last {
                    digits.insert(((begin, x-1), y), line[begin..x].parse::<usize>().unwrap());
                    last = false;
                }
            }
            x += 1;
        }
        if last {
            digits.insert(((begin, x), y), line[begin..x].parse::<usize>().unwrap());
            last = false;
        }
        y += 1;
    }

    // total sum
    let mut sum = 0;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),         (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    for &(sx, sy) in &symbols {
        for &(dx, dy) in &directions {
            let nx = sx as isize + dx;
            let ny = sy as isize + dy;
            if nx >= 0 && ny >= 0 {
                for &((begin_x, end_x), y) in digits.keys() {
                    if y == ny as usize && (begin_x..=end_x).contains(&(nx as usize)) {
                        if let Some(&digit) = digits.get(&((begin_x, end_x), y)) {
                            if visited.insert((begin_x, y)) {
                                sum += digit;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", sum)
}

fn two(contents: &String) {

    let mut symbols: HashSet<(usize, usize)> = HashSet::new();

    // numbers stored as (((beginX, endX), Y), Value)
    let mut digits: HashMap<((usize, usize), usize), usize> = HashMap::new(); 

    // Y coordinate in the grid
    let mut y: usize = 0;

    // keep track of the numbers
    let mut last = false;

    for line in contents.lines() {
        let mut x: usize = 0;
        let mut begin: usize = 0;
        for character in line.chars() {
            if character.is_digit(10) {
                if !last {
                    last = true;
                    begin = x;
                }
            }
            else {
                if character == '*' {
                    symbols.insert((x, y));
                }
                if last {
                    digits.insert(((begin, x-1), y), line[begin..x].parse::<usize>().unwrap());
                    last = false;
                }
            }
            x += 1;
        }
        if last {
            digits.insert(((begin, x), y), line[begin..x].parse::<usize>().unwrap());
            last = false;
        }
        y += 1;
    }

    // total sum
    let mut sum = 0;

    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),         (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    for &(sx, sy) in &symbols {
        let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
        for &(dx, dy) in &directions {
            let nx = sx as isize + dx;
            let ny = sy as isize + dy;
            if nx >= 0 && ny >= 0 {
                for &((begin_x, end_x), y) in digits.keys() {
                    if y == ny as usize && (begin_x..=end_x).contains(&(nx as usize)) {
                        if let Some(&digit) = digits.get(&((begin_x, end_x), y)) {
                            visited.insert((begin_x, y), digit);
                        }
                    }
                }
            }
        }
        if visited.len() == 2 {
            let mut mutliply = 1;
            for digit in visited.values() {
                mutliply *= digit;
            }
            sum += mutliply
        }
    }

    println!("{}", sum)
}