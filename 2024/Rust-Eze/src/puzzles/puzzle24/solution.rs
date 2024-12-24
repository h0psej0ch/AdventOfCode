use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() {
    let file_path = "src/puzzles/puzzle24/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let mut values: HashMap<String, bool> = HashMap::new();
    let mut gates: HashMap<String, (String, String, String)> = HashMap::new();

    let (know, determine) = contents.split_once("\n\n").unwrap();

    know.lines()
        .map(|line| line.split_once(": ").unwrap())
        .for_each(|(gate, value)| {
            values.insert(gate.to_string(), value == "1");
        });

    determine
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(value, key)| {
            (
                value
                    .split(" ")
                    .map(|str| str.to_string())
                    .collect_tuple::<(String, String, String)>()
                    .unwrap(),
                key,
            )
        })
        .for_each(|(values, key)| {
            gates.insert(key.parse().unwrap(), values);
        });

    let mut result: i64 = 0;
    let mut index = 0;

    loop {
        let new = format!("z{:02}", index);

        if !gates.contains_key(&new) {
            break;
        }

        if calculate(new, &values, &gates) {
            result += 1 << index;
        }

        index += 1;
    }

    println!("Puzzle One: {}", result);
}

fn calculate(
    value: String,
    values: &HashMap<String, bool>,
    gates: &HashMap<String, (String, String, String)>,
) -> bool {
    if values.contains_key(&value) {
        *values.get(&value).unwrap()
    } else {
        let (gate1, operator, gate2) = gates.get(&value).unwrap().clone();
        match operator.as_str() {
            "AND" => calculate(gate1, values, gates) & calculate(gate2, values, gates),
            "OR" => calculate(gate1, values, gates) | calculate(gate2, values, gates),
            "XOR" => calculate(gate1, values, gates) ^ calculate(gate2, values, gates),
            _ => {
                println!("TRIGGERED");
                false
            }
        }
    }
}

fn two(contents: &String) {
    let mut gates: HashMap<String, (String, String, String)> = HashMap::new();

    let (_know, determine) = contents.split_once("\n\n").unwrap();

    determine
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(value, key)| {
            (
                value
                    .split(" ")
                    .map(|str| str.to_string())
                    .collect_tuple::<(String, String, String)>()
                    .unwrap(),
                key,
            )
        })
        .for_each(|(values, key)| {
            gates.insert(key.parse().unwrap(), values);
        });

    let mut bads = Vec::new();

    gates
        .iter()
        .for_each(|(result_gate, (gate1, operator, gate2))| {
            if result_gate.starts_with("z") && operator != "XOR" && !(result_gate == "z45") {
                bads.push(result_gate)
            }
            match operator.as_str() {
                "AND" => {
                    if !(gate1 == "x00" || gate2 == "x00") {
                        for (_, (possible_parent1, parent_operator, possible_parent2)) in
                            gates.clone()
                        {
                            if (possible_parent1 == *result_gate
                                || possible_parent2 == *result_gate)
                                && !(parent_operator == "OR")
                            {
                                bads.push(result_gate);
                            }
                        }
                    }
                }
                "OR" => {}
                "XOR" => {
                    if (!result_gate.starts_with("z"))
                        && !(gate1.starts_with("x") && gate2.starts_with("y"))
                        && !(gate1.starts_with("y") && gate2.starts_with("x"))
                    {
                        bads.push(result_gate)
                    } else if gates.clone().iter().any(
                        |(_, (possible_parent1, parent_operator, possible_parent2))| {
                            (possible_parent1 == result_gate
                                || possible_parent2 == result_gate) && parent_operator == "OR"
                        },
                    ) {
                        bads.push(result_gate)
                    }
                }
                _ => {
                    println!("?")
                }
            }
        });

    bads.sort();
    bads.dedup();
    println!("Puzzle Two: {}", bads.iter().join(","))
}
