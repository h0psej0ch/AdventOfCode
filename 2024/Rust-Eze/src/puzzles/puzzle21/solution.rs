use std::collections::HashMap;
use itertools::Itertools;

pub fn solve() {
    let file_path = "src/puzzles/puzzle21/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let mut keypad = HashMap::new();

    keypad.insert('7', (0, 0));
    keypad.insert('8', (0, 1));
    keypad.insert('9', (0, 2));
    keypad.insert('4', (1, 0));
    keypad.insert('5', (1, 1));
    keypad.insert('6', (1, 2));
    keypad.insert('1', (2, 0));
    keypad.insert('2', (2, 1));
    keypad.insert('3', (2, 2));
    keypad.insert('0', (3, 1));
    keypad.insert('A', (3, 2));

    let mut robotpad = HashMap::new();
    robotpad.insert('^', (0, 1));
    robotpad.insert('v', (1, 1));
    robotpad.insert('<', (1, 0));
    robotpad.insert('>', (1, 2));
    robotpad.insert('A', (0, 2));

    let results = contents
        .lines()
        .map(|line| {
            let number = line[0..3].parse::<usize>().unwrap();
            let chars = line.chars().collect::<Vec<char>>();
            (number, chars)
        })
        .map(|(numeric, code)| {
            let mut steps = Vec::new();
            let origin: Vec<char> = Vec::new();
            steps.push(origin);
            let (mut y, mut x) = (3i32, 2i32);
            code.iter().for_each(|character| {
                let (ny, nx) = keypad.get(character).unwrap();
                let mut to_step = Vec::new();
                let mut append;
                if nx > &x {
                    append = '>'
                } else {
                    append = '<'
                }
                for _ in 0..x.abs_diff(*nx) {
                    to_step.push(append)
                }
                if ny > &y {
                    append = 'v'
                } else {
                    append = '^'
                }
                for _ in 0..y.abs_diff(*ny) {
                    to_step.push(append)
                }
                let mut steppers = to_step.iter().cloned().permutations(to_step.clone().len())
                    .filter(|perm| {
                        let mut temp_y = y;
                        let mut temp_x = x;
                        for &step in perm {
                            match step {
                                '>' => temp_x += 1,
                                '<' => temp_x -= 1,
                                '^' => temp_y -= 1,
                                'v' => temp_y += 1,
                                _ => {}
                            }
                            if temp_y == 3 && temp_x == 0 {
                                return false;
                            }
                        }
                        true
                    }).collect::<Vec<_>>();
                steppers.sort();
                steppers.dedup();

                let mut new_steps = Vec::new();
                steps.iter().for_each(|existing| {
                    steppers.iter().for_each(|additive| {
                        let mut combined = existing.clone();
                        combined.extend(additive.clone());
                        new_steps.push(combined);
                    })
                });
                new_steps.iter_mut().for_each(|vec| vec.push('A'));

                steps = new_steps;

                y = *ny;
                x = *nx;
            });
            (numeric, steps)
        })
        .map(|(numeric, mut code)| {
            let mut lowest = isize::MAX;
            let optimals = calculate_optimal();

            code.iter().for_each(|possibility| {

                let mut pairs: HashMap<(char, char), isize> = HashMap::new();

                let mut current = 'A';

                possibility.iter().for_each(|&character| {
                    *pairs.entry((current, character)).or_insert(0) += 1;
                    current = character;
                });
                for _ in 0..2 {
                    let mut new_pairs: HashMap<(char, char), isize> = HashMap::new();
                    for (key, value) in pairs.iter() {
                        for (a, b) in optimals.get(key).unwrap().iter().tuple_windows() {
                            *new_pairs.entry((*a, *b)).or_insert(0) += value;
                        }
                    }
                    pairs = new_pairs;
                }

                let count =  pairs.iter().map(|(a, b)| b).sum::<isize>();
                if count < lowest {
                    lowest = count;
                }
            });

            numeric * lowest as usize
        })
        .sum::<usize>();

    println!("Puzzle One: {:?}", results)
}

fn calculate_optimal() -> HashMap<(char, char), Vec<char>> {

    let mut robotpad = HashMap::new();
    robotpad.insert('^', (0, 1));
    robotpad.insert('v', (1, 1));
    robotpad.insert('<', (1, 0));
    robotpad.insert('>', (1, 2));
    robotpad.insert('A', (0, 2));

    let mut optimals: HashMap<(char, char), Vec<char>> = HashMap::new();

    let characters: [char ; 5] = ['^', 'v', '<', '>', 'A'];

    characters.iter().for_each(|&char| { optimals.insert((char, char), vec!['A', 'A']); });

    optimals.insert(('A', 'v'), vec!['A', '<', 'v', 'A']);      //
    optimals.insert(('A', '^'), vec!['A', '<', 'A']);           //
    optimals.insert(('A', '<'), vec!['A', 'v', '<', '<', 'A']); //
    optimals.insert(('A', '>'), vec!['A', 'v', 'A']);           //

    optimals.insert(('v', 'A'), vec!['A', '^', '>', 'A']);      //
    optimals.insert(('v', '<'), vec!['A', '<', 'A']);           //
    optimals.insert(('v', '>'), vec!['A', '>', 'A']);           //

    optimals.insert(('^', 'A'), vec!['A', '>', 'A']);           //
    optimals.insert(('^', '<'), vec!['A', 'v', '<', 'A']);      //
    optimals.insert(('^', '>'), vec!['A', 'v', '>', 'A']);      //

    optimals.insert(('<', 'v'), vec!['A', '>', 'A']);           //
    optimals.insert(('<', '^'), vec!['A', '>', '^', 'A']);      //
    optimals.insert(('<', 'A'), vec!['A', '>', '>', '^', 'A']); //

    optimals.insert(('>', 'v'), vec!['A', '<', 'A']);           //
    optimals.insert(('>', '^'), vec!['A', '<', '^', 'A']);      //
    optimals.insert(('>', 'A'), vec!['A', '^', 'A']);           //

    optimals
}

fn two(contents: &String) {
    let mut keypad = HashMap::new();

    keypad.insert('7', (0, 0));
    keypad.insert('8', (0, 1));
    keypad.insert('9', (0, 2));
    keypad.insert('4', (1, 0));
    keypad.insert('5', (1, 1));
    keypad.insert('6', (1, 2));
    keypad.insert('1', (2, 0));
    keypad.insert('2', (2, 1));
    keypad.insert('3', (2, 2));
    keypad.insert('0', (3, 1));
    keypad.insert('A', (3, 2));

    let mut robotpad = HashMap::new();
    robotpad.insert('^', (0, 1));
    robotpad.insert('v', (1, 1));
    robotpad.insert('<', (1, 0));
    robotpad.insert('>', (1, 2));
    robotpad.insert('A', (0, 2));

    let results = contents
        .lines()
        .map(|line| {
            let number = line[0..3].parse::<usize>().unwrap();
            let chars = line.chars().collect::<Vec<char>>();
            (number, chars)
        })
        .map(|(numeric, code)| {
            let mut steps = Vec::new();
            let origin: Vec<char> = Vec::new();
            steps.push(origin);
            let (mut y, mut x) = (3i32, 2i32);
            code.iter().for_each(|character| {
                let (ny, nx) = keypad.get(character).unwrap();
                let mut to_step = Vec::new();
                let mut append;
                if nx > &x {
                    append = '>'
                } else {
                    append = '<'
                }
                for _ in 0..x.abs_diff(*nx) {
                    to_step.push(append)
                }
                if ny > &y {
                    append = 'v'
                } else {
                    append = '^'
                }
                for _ in 0..y.abs_diff(*ny) {
                    to_step.push(append)
                }
                let mut steppers = to_step.iter().cloned().permutations(to_step.clone().len())
                    .filter(|perm| {
                        let mut temp_y = y;
                        let mut temp_x = x;
                        for &step in perm {
                            match step {
                                '>' => temp_x += 1,
                                '<' => temp_x -= 1,
                                '^' => temp_y -= 1,
                                'v' => temp_y += 1,
                                _ => {}
                            }
                            if temp_y == 3 && temp_x == 0 {
                                return false;
                            }
                        }
                        true
                    }).collect::<Vec<_>>();
                steppers.sort();
                steppers.dedup();

                let mut new_steps = Vec::new();
                steps.iter().for_each(|existing| {
                    steppers.iter().for_each(|additive| {
                        let mut combined = existing.clone();
                        combined.extend(additive.clone());
                        new_steps.push(combined);
                    })
                });
                new_steps.iter_mut().for_each(|vec| vec.push('A'));

                steps = new_steps;

                y = *ny;
                x = *nx;
            });
            (numeric, steps)
        })
        .map(|(numeric, mut code)| {
            let mut lowest = isize::MAX;
            let optimals = calculate_optimal();

            code.iter().for_each(|possibility| {

                let mut pairs: HashMap<(char, char), isize> = HashMap::new();

                let mut current = 'A';

                possibility.iter().for_each(|&character| {
                    *pairs.entry((current, character)).or_insert(0) += 1;
                    current = character;
                });
                for _ in 0..25 {
                    let mut new_pairs: HashMap<(char, char), isize> = HashMap::new();
                    for (key, value) in pairs.iter() {
                        for (a, b) in optimals.get(key).unwrap().iter().tuple_windows() {
                            *new_pairs.entry((*a, *b)).or_insert(0) += value;
                        }
                    }
                    pairs = new_pairs;
                }

                let count =  pairs.iter().map(|(a, b)| b).sum::<isize>();
                if count < lowest {
                    lowest = count;
                }
            });

            numeric * lowest as usize
        })
        .sum::<usize>();

    println!("Puzzle Two: {:?}", results)
}
