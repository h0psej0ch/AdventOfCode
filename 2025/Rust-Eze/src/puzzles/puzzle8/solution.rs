use std::collections::HashSet;

pub fn solve() {
    let file_path = "src/puzzles/puzzle8/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");
    let s = Solver::new(contents);
    s.one();
    s.two();
}

struct Solver {
    distances: Vec<(isize, usize, usize)>,
    junctions: Vec<(isize, isize, isize)>,
}

impl Solver {
    fn new(contents: String) -> Self {
        let junctions = contents
            .lines()
            .map(|line| line.split(',').collect::<Vec<&str>>())
            .map(|x| {
                (
                    x[0].parse().unwrap(),
                    x[1].parse().unwrap(),
                    x[2].parse().unwrap(),
                )
            })
            .collect::<Vec<(isize, isize, isize)>>();
        let mut distances: Vec<(isize, usize, usize)> = Vec::new();
        junctions.iter().enumerate().for_each(|(i, junc1)| {
            for j in i + 1..junctions.len() {
                let junc2 = junctions[j];
                distances.push((
                    ((junc1.0 - junc2.0).pow(2)
                        + (junc1.1 - junc2.1).pow(2)
                        + (junc1.2 - junc2.2).pow(2))
                    .isqrt(),
                    i,
                    j,
                ));
            }
        });
        distances.sort_by(|a, b| (a.0).cmp(&b.0));
        Self {
            distances,
            junctions,
        }
    }
    fn one(&self) {
        let mut networks: Vec<HashSet<usize>> = Vec::new();

        self.distances.iter().take(1000).for_each(|(_, n, m)| {
            let net1_idx = networks.iter().position(|x| x.contains(n));
            let net2_idx = networks.iter().position(|x| x.contains(m));

            if !(net1_idx == net2_idx) || (net1_idx == None) {
                let mut set1 = if let Some(idx) = net1_idx {
                    networks.remove(idx)
                } else {
                    HashSet::from([*n])
                };

                let mut set2 = if let Some(mut idx) = net2_idx {
                    if let Some(net1_idx) = net1_idx {
                        if idx >= net1_idx {
                            idx -= 1;
                        }
                    }
                    networks.remove(idx)
                } else {
                    HashSet::from([*m])
                };

                set1.extend(set2.drain());
                networks.push(set1);
            }
        });
        networks.sort_by_key(|a| std::cmp::Reverse(a.len()));
        println!(
            "Puzzle 8.1: {}",
            networks[0].len() * networks[1].len() * networks[2].len()
        );
    }

    fn two(&self) {
        let mut networks: Vec<HashSet<usize>> = Vec::new();
        let mut changed = false;
        let mut last = (0, 0);
        for (_, n, m) in self.distances.iter() {
            if networks.len() <= 1 && changed && networks[0].len() == self.junctions.len() {
                break;
            }
            let net1_idx = networks.iter().position(|x| x.contains(n));
            let net2_idx = networks.iter().position(|x| x.contains(m));

            if !(net1_idx == net2_idx) || (net1_idx == None) {
                let mut set1 = if let Some(idx) = net1_idx {
                    networks.remove(idx)
                } else {
                    HashSet::from([*n])
                };

                let mut set2 = if let Some(mut idx) = net2_idx {
                    if let Some(net1_idx) = net1_idx {
                        if idx >= net1_idx {
                            idx -= 1;
                        }
                    }
                    networks.remove(idx)
                } else {
                    HashSet::from([*m])
                };

                set1.extend(set2.drain());
                networks.push(set1);
                if networks.len() > 1 {
                    changed = true;
                }
                last = (*n, *m);
            }
        }
        println!(
            "Puzzel 8.2: {}",
            self.junctions[last.0].0 * self.junctions[last.1].0
        );
    }
}
