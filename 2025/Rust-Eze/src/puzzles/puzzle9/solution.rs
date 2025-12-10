use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() {
    let file_path = "src/puzzles/puzzle9/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");
    let s = Solver::new(contents);
    s.one();
    s.two();
}

struct Solver {
    reds: Vec<(isize, isize)>,
}

impl Solver {
    fn new(contents: String) -> Self {
        Self {
            reds: contents
                .lines()
                .map(|line| line.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
                .collect(),
        }
    }

    fn one(&self) {
        let mut max = 0;
        for i in 0..self.reds.len() {
            for j in i + 1..self.reds.len() {
                let redi = self.reds[i];
                let redj = self.reds[j];
                let distance = ((redj.0 - redi.0).abs() + 1) * ((redj.1 - redi.1).abs() + 1);
                if distance > max {
                    max = distance;
                }
            }
        }
        println!("Puzzle 9.1: {}", max)
    }

    fn two(&self) {
        let mut new_reds = self.reds.clone();
        new_reds.push(new_reds[0]);
        let mut vertical: HashMap<isize, Vec<(isize, isize)>> = HashMap::new();
        for (&a, &b) in new_reds
            .iter()
            .tuple_windows()
            .filter(|(&a, &b)| a.0 == b.0)
        {
            vertical
                .entry(a.0)
                .or_default()
                .push((std::cmp::min(a.1, b.1), std::cmp::max(a.1, b.1)));
        }
        let mut horizontal: HashMap<isize, Vec<(isize, isize)>> = HashMap::new();
        for (&a, &b) in new_reds
            .iter()
            .tuple_windows()
            .filter(|(&a, &b)| a.1 == b.1)
        {
            horizontal
                .entry(a.1)
                .or_default()
                .push((std::cmp::min(a.0, b.0), std::cmp::max(a.0, b.0)));
        }

        let mut max = 0;
        let mut comb = self.reds.iter().combinations(2).collect::<Vec<Vec<_>>>();
        comb.sort_by(|v1, v2| {
            let c: (isize, isize) = *v1[0];
            let d: (isize, isize) = *v1[1];
            let a: (isize, isize) = *v2[0];
            let b: (isize, isize) = *v2[1];
            (((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1))
                .cmp(&(((c.0 - d.0).abs() + 1) * ((c.1 - d.1).abs() + 1)))
        });
        for v in comb {
            let redi = v[0];
            let redj = v[1];
            let distance = ((redj.0 - redi.0).abs() + 1) * ((redj.1 - redi.1).abs() + 1);
            if distance > max {
                let lowx = std::cmp::min(redi.0, redj.0);
                let lowy = std::cmp::min(redi.1, redj.1);
                let highx = std::cmp::max(redi.0, redj.0);
                let highy = std::cmp::max(redi.1, redj.1);
                if (lowx + 1..highx).all(|x| match vertical.get(&x) {
                    Some(v) => v.iter().all(|&(cy1, cy2)| cy2 <= lowy || cy1 >= highy),
                    None => true,
                }) && (lowy + 1..highy).all(|y| match horizontal.get(&y) {
                    Some(v) => v.iter().all(|&(cx1, cx2)| cx2 <= lowx || cx1 >= highx),
                    None => true,
                }) {
                    max = distance;
                }
            }
        }

        println!("Puzzle 9.2: {}", max);
    }
}
