use std::collections::HashMap;

pub fn solve() {
    
    let file_path = "src/puzzles/puzzle1/input.txt";

    let contents = std::fs::read_to_string(file_path)
        .expect("No input file");

    let mut solver = Solver::new(contents);

    solver.one();
    solver.two();

}

struct Solver {
    list1: Vec<usize>,
    list2: Vec<usize>,
    occurrences: HashMap<usize, usize>
}

impl Solver {
    fn new(contents: String) -> Solver {
        let mut list1: Vec<usize> = Vec::new();
        let mut list2: Vec<usize> = Vec::new();

        for line in contents.lines() {
            let parts: Vec<&str> = line.split("   ").collect();
            list1.push((parts[0]).parse::<usize>().unwrap());
            list2.push((parts[1]).parse::<usize>().unwrap());
        }

        let occurrences: HashMap<usize, usize> = HashMap::new();

        Solver { list1, list2, occurrences }
    }

    fn one(&mut self) {
        self.list1.sort();
        self.list2.sort();

        let mut result: usize = 0;

        for i in 0 ..self.list1.len() {
            if self.list1[i] > self.list2[i] {
                result += self.list1[i] - self.list2[i];
            } else {
                result += self.list2[i] - self.list1[i];
            }
        }
        println!("Puzzle One: {}", result)
    }

    fn two(&mut self) {
        let mut result: usize = 0;

        for i in 0 ..  self.list2.len() {
            if self.occurrences.contains_key(&self.list2[i]) {
                *(self.occurrences.get_mut(&self.list2[i]).unwrap()) += 1;
            } else {
                self.occurrences.insert(self.list2[i], 1);
            }
        }

        for i in 0 .. self.list1.len() {
            if self.occurrences.contains_key(&self.list1[i]) {
                result += *(self.occurrences.get(&self.list1[i]).unwrap()) * self.list1[i];
            }
        }

        println!("Puzzle Two: {}", result)
    }
}

