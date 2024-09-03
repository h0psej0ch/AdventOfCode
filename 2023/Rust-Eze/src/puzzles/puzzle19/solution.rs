use core::panic;
use std::collections::{HashMap, VecDeque};

pub fn solve() {
    let file_path = "src/puzzles/puzzle19/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    let mut solver = Solver::new(&contents);

    solver.one();
    solver.two();
}

struct Solver {
    rules: HashMap<String, Vec<String>>,
    input: Vec<HashMap<i32, i32>>,
}

impl Solver {
    fn new(contents: &str) -> Self {
        let split: Vec<_> = contents.split("\n\n").collect();

        println!("{}", split[0]);

        let mut rules = HashMap::new();

        for rule in split[0].lines() {
            let new_split: Vec<_> = rule.split("{").collect();
            println!("{}", &(new_split[1]).to_string()[0..new_split[1].len() - 1]);
            rules.insert(
                (new_split[0]).to_string(),
                (new_split[1][0..new_split[1].len() - 1])
                    .split(",")
                    .map(|s| s.to_string())
                    .collect(),
            );
        }

        println!("{:?}", rules);

        let mut input: Vec<HashMap<i32, i32>> = Vec::new();

        for input_set in split[1].lines() {
            let new_split = input_set[1..input_set.len() - 1].split(",");
            let mut input_map: HashMap<i32, i32> = HashMap::new();
            for value in new_split {
                let ripped: Vec<_> = value.split("=").collect();
                match ripped[0] {
                    "x" => {
                        input_map.insert(0, ripped[1].parse().unwrap());
                    }
                    "m" => {
                        input_map.insert(1, ripped[1].parse().unwrap());
                    }
                    "a" => {
                        input_map.insert(2, ripped[1].parse().unwrap());
                    }
                    "s" => {
                        input_map.insert(3, ripped[1].parse().unwrap());
                    }
                    _ => println!("huh?"),
                }
            }
            input.push(input_map)
        }

        Self { rules, input }
    }

    fn one(&mut self) {
        let mut total: i32 = 0;
        for entry in &self.input {
            let mut current = "in";
            let mut step = 0;
            let mut evaluated = false;
            let mut accepted = false;
            while !evaluated {
                match self.rules.get(current) {
                    Some(check) => {
                        let rule = &check[step];
                        println!("{}", rule);
                        match rule.as_str() {
                            "A" => {
                                accepted = true;
                                evaluated = true;
                            }
                            "R" => {
                                evaluated = true;
                            }
                            _ => {
                                let split_rule: Vec<_> = rule.split(":").collect();
                                if split_rule.len() == 1 {
                                    current = split_rule[0];
                                    step = 0;
                                    continue
                                }
                                let rule_check = split_rule[0].to_string();
                                let target = &rule_check[0..1];
                                let operator = &rule_check[1..2];
                                let value: i32 = rule_check[2..].parse().unwrap();
                                let target_value;
                                let mut passed: bool = false;
                                match target {
                                    "x" => { target_value = entry.get(&0).unwrap(); }
                                    "m" => { target_value = entry.get(&1).unwrap(); }
                                    "a" => { target_value = entry.get(&2).unwrap(); }
                                    "s" => { target_value = entry.get(&3).unwrap(); }
                                    _ => { panic!("Not a good value, hmmm") }
                                }
                                match operator {
                                    ">" => { if target_value > &value { passed = true; } }
                                    "<" => { if target_value < &value { passed = true; } }
                                    _ => { panic!("No operator?") }
                                }
                                if passed {
                                    match split_rule[1] {
                                        "A" => {
                                            accepted = true;
                                            evaluated = true;
                                        }
                                        "R" => {
                                            evaluated = true;
                                        }
                                        _ => { current = split_rule[1]; step = 0 }
                                    }
                                } else { step += 1; }
                            }
                        }
                    }
                    None => panic!("huh?")
                }
            }
            if accepted {
                total += entry.get(&0).unwrap() + entry.get(&1).unwrap() + entry.get(&2).unwrap() + entry.get(&3).unwrap()
            }
        }
        println!("total: {}", total);
    }

    fn two(&mut self) {
        let mut accepted: Vec<HashMap<i32, (i32, i32)>> =  Vec::new();
        let mut queue: VecDeque<(&str, usize, HashMap<i32, (i32, i32)>)> = VecDeque::new();
        let mut begin_values: HashMap<i32, (i32, i32)> = HashMap::new();
        for i in 0..=3 {
            begin_values.insert(i, (1, 4000));
        }
        queue.push_back(("in", 0, begin_values));

        while queue.len() > 0 {
            let current_entry = queue.pop_front().unwrap();
            let current_map = current_entry.2;
            match self.rules.get(current_entry.0) {
                Some(current_rule) => {
                    let rule = &current_rule[current_entry.1];
                    match rule.as_str() {
                        "A" => {
                            accepted.push(current_map);
                        }
                        "R" => {
                            continue
                        }
                        _ => {
                            let split_rule: Vec<_> = rule.split(":").collect();
                            if split_rule.len() == 1 {
                                queue.push_back((rule, 0, current_map));
                            } else {
                                let rule_check = split_rule[0].to_string();
                                let target = &rule_check[0..1];
                                let operator = &rule_check[1..2];
                                let value: i32 = rule_check[2..].parse().unwrap();
                                let target_value;
                                let bigger_then: bool;
                                match target {
                                    "x" => { target_value = 0; }
                                    "m" => { target_value = 1; }
                                    "a" => { target_value = 2; }
                                    "s" => { target_value = 3; }
                                    _ => { panic!("Not a good value, hmmm") }
                                }
                                match operator {
                                    ">" => { bigger_then = true; }
                                    _ => { bigger_then = false;}
                                }
                                let old_values = current_map.get(&target_value).unwrap();
                                if value > old_values.0 && value < old_values.1 {
                                    let mut clone_true = current_map.clone();
                                    let mut clone_false = current_map.clone();
                                    if bigger_then {
                                        clone_true.insert(target_value, (value + 1, old_values.1));
                                        clone_false.insert(target_value, (old_values.0, value));
                                    } else {
                                        clone_true.insert(target_value, (old_values.0, value - 1));
                                        clone_false.insert(target_value, (value, old_values.1));
                                    }
                                    queue.push_back((current_entry.0, current_entry.1 + 1, clone_false));

                                    match split_rule[1] {
                                        "A" => {
                                            accepted.push(clone_true);
                                        },
                                        "R" => {
                                            continue;
                                        }
                                        _ => {
                                            queue.push_back((split_rule[1], 0, clone_true));
                                        }
                                    }

                                } else {
                                    if (bigger_then && old_values.0 > value) || (!bigger_then && old_values.1 < value) {
                                        match split_rule[1] {
                                            "A" => {
                                                accepted.push(current_map);
                                            },
                                            "R" => {
                                                continue;
                                            }
                                            _ => {
                                                queue.push_back((split_rule[1], 0, current_map));
                                            }
                                        }
                                    } else { queue.push_back((current_entry.0, current_entry.1 + 1, current_map)); }
                                }
                            }
                        }
                    }
                }
                None => panic!("Hmmm no corresponding rule, big sad")
            }
        }

        println!("hihi: {:?}", accepted);

        let mut total: u128 = 0;
        for hashmap in accepted {
            let x = hashmap.get(&0).unwrap();
            let m = hashmap.get(&1).unwrap();
            let a = hashmap.get(&2).unwrap();
            let s = hashmap.get(&3).unwrap();
            let x = (x.1 - x.0 + 1) as u128;
            let m = (m.1 - m.0 + 1) as u128;
            let a = (a.1 - a.0 + 1) as u128;
            let s = (s.1 - s.0 + 1) as u128;
            println!("x: {}, m: {}, a: {}, s: {}", x, m, a, s);
            total += x * m * a * s;
        }
        println!("Total possibel inputs: {}", total);
    }
}
