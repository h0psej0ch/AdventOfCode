use good_lp::*;

pub fn solve() {
    let file_path = "src/puzzles/puzzle10/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");
    let s = Solver::new(contents);
    s.one();
    s.two();
}

struct Solver {
    problems: Vec<(Vec<i32>, Vec<Vec<i32>>, Vec<i32>)>,
}

impl Solver {
    fn new(contents: String) -> Self {
        Self {
            problems: contents
                .lines()
                .map(|line| line.split_whitespace().collect::<Vec<&str>>())
                .map(|splitup: Vec<&str>| {
                    let light_len = splitup[0].len();
                    (
                        splitup[0][1..light_len - 1]
                            .chars()
                            .map(|char| if char == '#' { 1_i32 } else { 0_i32 })
                            .collect::<Vec<i32>>(),
                        splitup
                            .iter()
                            .take(splitup.len() - 1)
                            .skip(1)
                            .map(|wiring| {
                                wiring[1..wiring.len() - 1]
                                    .split(",")
                                    .map(|num| num.parse::<usize>().unwrap())
                                    .collect::<Vec<_>>()
                            })
                            .map(|num_line| {
                                let mut current = Vec::new();
                                for i in 0..light_len - 2 {
                                    if num_line.contains(&i) {
                                        current.push(1_i32);
                                    } else {
                                        current.push(0_i32);
                                    }
                                }
                                current
                            })
                            .collect::<Vec<_>>(),
                        splitup[splitup.len() - 1][1..splitup[splitup.len() - 1].len() - 1]
                            .split(",")
                            .map(|num| num.parse::<i32>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>(),
        }
    }

    fn one(&self) {
        println!(
            "Puzzle 10.1: {}",
            self.problems
                .iter()
                .map(|(target, switches, _)| {
                    let mut min: i32 = switches.len() as i32;
                    let n = switches.len();
                    for mask in 0..(1 << n) {
                        let vec: Vec<i32> = (0..n).map(|i| ((mask >> i) & 1) as i32).collect();
                        if *target
                            == switches
                                .iter()
                                .enumerate()
                                .filter(|(n, _)| vec[*n] == 1)
                                .fold(vec![0; switches[0].len()], |acc, (_, next)| {
                                    let mut res = Vec::new();
                                    for (n, x) in acc.iter().enumerate() {
                                        res.push(x ^ next[n]);
                                    }
                                    res
                                })
                            && vec.iter().sum::<i32>() < min
                        {
                            min = vec.iter().sum::<i32>();
                        };
                    }
                    min
                })
                .sum::<i32>()
        );
    }

    fn two(&self) {
        println!(
            "Puzzle 10.2: {}",
            self.problems
                .iter()
                .map(|(_, switches, target)| {
                    let mut vars = variables!();
                    let var_vec = (0..switches.len())
                        .map(|_| vars.add(variable().integer().min(0)))
                        .collect::<Vec<Variable>>();
                    let mut model = vars
                        .minimise(var_vec.iter().copied().sum::<Expression>())
                        .using(coin_cbc);

                    for n in 0..target.len() {
                        let col_sum: Expression = (0..switches.len())
                            .map(|x| var_vec[x] * switches[x][n])
                            .sum();
                        model = model.with(constraint!(col_sum == target[n]));
                    }
                    let solution = model.solve().unwrap();
                    solution.eval(var_vec.iter().copied().sum::<Expression>()) as i32
                })
                .sum::<i32>()
        );
    }
}
