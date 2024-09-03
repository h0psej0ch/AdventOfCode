use std::collections::{HashMap, VecDeque};

pub fn solve() {
    let file_path = "src/puzzles/puzzle20/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    let mut solver = Solver::new(contents);

    solver.one();
    solver.two();
}

struct Solver {
    flip_flop: HashMap<String, (bool, Vec<String>)>,
    conjunction: HashMap<String, (HashMap<String, bool>, Vec<String>)>,
}

impl Solver {
    fn new(contents: String) -> Solver {
        let mut flip_flop = HashMap::new();
        let mut conjunction = HashMap::new();

        for line in contents.lines() {
            let split_line: Vec<_> = line.split(" -> ").collect();
            let destinations: Vec<_> = split_line[1].split(", ").map(|s| s.to_string()).collect();
            match line.chars().nth(0).unwrap() {
                '&' => {
                    let mut input: HashMap<String, bool> = HashMap::new();
                    conjunction.insert(split_line[0][1..].to_string(), (input, destinations));
                }
                '%' | 'b' => {
                    flip_flop.insert(split_line[0][1..].to_string(), (false, destinations));
                }
                _ => {}
            }
        }

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

        println!("{:?}", flip_flop);
        for i in 0..5 {println!()};
        println!("{:?}", conjunction);

        Solver {
            flip_flop,
            conjunction,
        }
    }

    fn one(&mut self) {

        let mut queue: VecDeque<(String, bool)> = VecDeque::new();

        let mut low: usize = 1;
        let mut high: usize = 0;

        // queue.push_back(("roadcaster".to_string(), false));

        for node in &self.flip_flop.get("roadcaster").unwrap().1 {
            queue.push_back((node.to_string(), false))
        }

        println!("{}", self.flip_flop.keys().all(|key| !(self.flip_flop.get(key).unwrap().0)));

        let mut first = true;

        // Improve: Push multiple times for better result if queue is empty :)

        while (!self.flip_flop.keys().all(|key| !(self.flip_flop.get(key).unwrap().0)) && queue.len() > 0) || first {
            let current_node = queue.pop_front().unwrap_or(("".to_string(), false));
            if current_node.1 { high += 1} else { low += 1 };
            match self.flip_flop.get_mut(&current_node.0) {
                Some(flip) => {
                    println!("flip");
                    println!("{}", current_node.1);
                    if !current_node.1 {
                        flip.0 = !flip.0;
                        println!("{:?}", flip.1);
                        for node in &flip.1 {
                            println!("{}", node);
                            queue.push_back((node.to_string(), flip.0));
                        }
                    }
                }
                None => {
                    match self.conjunction.get_mut(&current_node.0) {
                        Some(conj) => {
                            println!("conj");
                            conj.0.insert(current_node.0, current_node.1);
                            for node in &conj.1 {
                                queue.push_back((node.to_string(), conj.0.keys().all(|key| *conj.0.get(key).unwrap_or(&false))))
                            }
                        }
                        None => {}
                    }
                }

            } 
            if first { first = !first }
            println!("{:?}", queue)
        }

        // TODO implement teehee

        println!("{}, {}", low, high);


    }

    fn two(&self) {}

}
