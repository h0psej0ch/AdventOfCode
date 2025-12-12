use std::collections::{HashMap, VecDeque};

pub fn solve() {
    let file_path = "src/puzzles/puzzle20/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    let mut solver = Solver::new(contents);

    solver.solve();
}

struct Solver {
    flip_flop: HashMap<String, (bool, Vec<String>)>,
    conjunction: HashMap<String, (HashMap<String, bool>, Vec<String>)>,
}

impl Solver {
    fn new(contents: String) -> Solver {
        let mut flip_flop = HashMap::new();
        let mut conjunction = HashMap::new();

        contents
            .lines()
            .map(|line| line.split_once(" -> ").unwrap())
            .for_each(|(source, dest)| match source.chars().nth(0).unwrap() {
                '&' => {
                    conjunction.insert(
                        source[1..].to_string(),
                        (
                            HashMap::new(),
                            dest.split(", ").map(|s| s.to_string()).collect(),
                        ),
                    );
                }
                '%' | 'b' => {
                    flip_flop.insert(
                        source[1..].to_string(),
                        (
                            false,
                            dest.split(",")
                                .map(|s| s.to_string().split_whitespace().collect::<String>())
                                .collect(),
                        ),
                    );
                }
                _ => {}
            });

        for key in flip_flop.keys() {
            for destination in &flip_flop.get(key).unwrap().1 {
                match conjunction.get_mut(destination) {
                    Some(conj) => {
                        conj.0.insert(key.to_string(), false);
                    }
                    None => {}
                }
            }
        }
        let conjunction_copy = conjunction.clone();
        for key in conjunction_copy.keys() {
            for destination in &conjunction_copy.get(key).unwrap().1 {
                match conjunction.get_mut(destination) {
                    Some(conj) => {
                        conj.0.insert(key.to_string(), false);
                    }
                    None => {}
                }
            }
        }

        Solver {
            flip_flop,
            conjunction,
        }
    }

    fn solve(&mut self) {
        let mut responsibles: Vec<String> = self
            .conjunction
            .get("dt")
            .unwrap()
            .0
            .keys()
            .map(|x| x.to_string())
            .collect();

        let mut resp_map: HashMap<String, usize> = HashMap::new();

        let mut button_presses = 0;

        let mut low = 0;
        let mut high = 0;

        while !responsibles.is_empty() {
            button_presses += 1;
            if button_presses == 1001 {
                println!("Puzzle 20.1: {}", low * high);
            }
            let length = self.flip_flop.get("roadcaster").unwrap().1.len();
            low += 1;
            let mut signals: VecDeque<((String, String), bool)> = VecDeque::from(
                self.flip_flop
                    .get("roadcaster")
                    .unwrap()
                    .1
                    .clone()
                    .into_iter()
                    .zip(vec!["roadcaster".to_string(); length])
                    .zip(vec![false; length])
                    .collect::<Vec<((String, String), bool)>>(),
            );
            while !signals.is_empty() {
                let ((current, from), sig) = signals.pop_front().unwrap();
                if sig {
                    high += 1;
                } else {
                    low += 1;
                }
                if sig && responsibles.contains(&from) {
                    match resp_map.get(&from) {
                        Some(val) => {
                            resp_map.insert(from.clone(), button_presses - val);
                            responsibles
                                .remove(responsibles.iter().position(|n| *n == from).unwrap());
                        }
                        None => {
                            resp_map.insert(from.clone(), button_presses);
                        }
                    }
                }
                if current != "rx" && current != "output" {
                    match self.flip_flop.get_mut(&current) {
                        Some((active, further)) => {
                            if !sig {
                                *active = !*active;
                                signals.extend(
                                    further
                                        .iter()
                                        .map(|x| x.to_string())
                                        .zip(vec![current; further.len()])
                                        .zip(vec![*active; further.len()])
                                        .collect::<Vec<((String, String), bool)>>(),
                                );
                            }
                        }
                        None => {
                            let (received, further) = self.conjunction.get_mut(&current).unwrap();
                            received.insert(from, sig);
                            signals.extend(
                                further
                                    .iter()
                                    .map(|x| x.to_string())
                                    .zip(vec![current; further.len()])
                                    .zip(vec![!received.values().all(|val| *val); further.len()])
                                    .collect::<Vec<((String, String), bool)>>(),
                            );
                        }
                    }
                }
            }
        }
        let divisor = resp_map
            .values()
            .copied()
            .reduce(|acc, x| gcf(acc, x))
            .unwrap();
        println!(
            "Puzzle 20.2: {}",
            resp_map.values().copied().reduce(|acc, x| acc * x).unwrap() / divisor
        );
    }
}

fn gcf(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}
