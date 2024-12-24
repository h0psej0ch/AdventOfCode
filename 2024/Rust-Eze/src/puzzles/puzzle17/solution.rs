use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve() {
    let file_path = "src/puzzles/puzzle17/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let mut registers: HashMap<char, i64> = HashMap::new();
    let (reg, program) = contents.split_once("\n\n").unwrap();

    reg.lines()
        .map(|line| (&line[9..10], &line[12..line.len()]))
        .for_each(|(char, value)| {
            registers.insert(char.chars().next().unwrap(), value.parse::<i64>().unwrap());
        });

    let instructions: Vec<(_, _)> = program
        .split_once(" ")
        .unwrap()
        .1
        .split(",")
        .tuples()
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    let output = calculate_output(&instructions, registers);

    println!("Puzzle One: {}", output.iter().join(","))
}

fn two(contents: &String) {
    let (_reg, program) = contents.split_once("\n\n").unwrap();
    let instructions_raw: Vec<i64> = program
        .split_once(" ")
        .unwrap()
        .1
        .split(",").map(|string| string.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let instructions = instructions_raw.clone().into_iter()
        .tuples()
        .collect::<Vec<_>>();

    let mut results: HashSet<i64> = HashSet::new();
    recursion(0, 0, &instructions_raw, &instructions, &mut results);

    println!("Puzzle Two: {}", results.iter().min().unwrap());
}

fn recursion(a: i64, index: usize, instructions_raw: &Vec<i64>, instructions: &Vec<(i64, i64)>, solutions: &mut HashSet<i64>) {
    let mut map = HashMap::new();
    map.insert('A', a);
    map.insert('B', 0);
    map.insert('C', 0);
    let output = calculate_output(instructions, map);
    if output == *instructions_raw {
        solutions.insert(a);
    }
    else if index == 0 || output == instructions_raw[instructions_raw.len()-index..instructions_raw.len()] {
        for i in 0..8 {
            recursion(8 * a + i, index + 1, instructions_raw, instructions, solutions)
        }
    }
}

fn calculate_output(instructions: &Vec<(i64, i64)>, mut registers: HashMap<char, i64>) -> Vec<i64>{
    let mut output = Vec::new();

    let mut index = 0;
    while index < instructions.len() {
        let (opcode, operand) = instructions[index];
        let combo;
        match operand {
            4 => { combo = *registers.get(&'A').unwrap(); }
            5 => { combo = *registers.get(&'B').unwrap(); }
            6 => { combo = *registers.get(&'C').unwrap(); }
            _ => { combo = operand; }
        }


        match opcode {
            0 => {
                *registers.get_mut(&'A').unwrap() >>= combo;
            }
            1 => {
                *registers.get_mut(&'B').unwrap() ^= operand;
            }
            2 => {
                registers.insert('B', combo & 7);
            }
            3 => {
                if *registers.get(&'A').unwrap() != 0 {
                    index = operand as usize / 2;
                } else {
                    index += 1;
                }
            }
            4 => {
                *registers.get_mut(&'B').unwrap() ^= *registers.get(&'C').unwrap();
            }
            5 => { output.push(combo & 7) }
            6 => { registers.insert('B', registers.get(&'A').unwrap() >> combo); }
            7 => { registers.insert('C', registers.get(&'A').unwrap() >> combo); }
            _ => {}
        }
        if opcode != 3 {
            index += 1;
        }
    };
    output
}
