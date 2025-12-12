pub fn solve() {
    let file_path = "src/puzzles/puzzle12/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");
    let s = Solver::new(contents);
    s.one();
}

struct Solver {
    blocks: Vec<Vec<bool>>,
    regions: Vec<((usize, usize), Vec<usize>)>,
}

impl Solver {
    fn new(contents: String) -> Self {
        let splitup = contents.split("\n\n").collect::<Vec<_>>();
        Self {
            blocks: splitup
                .iter()
                .take(splitup.len() - 1)
                .map(|block| {
                    block
                        .lines()
                        .skip(1)
                        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
                        .fold(Vec::new(), |mut acc, x| {
                            acc.extend(x);
                            acc
                        })
                })
                .collect::<Vec<Vec<bool>>>(),
            regions: splitup[splitup.len() - 1]
                .lines()
                .map(|line| line.split_once(": ").unwrap())
                .map(|(size, usage)| {
                    (
                        size.split_once("x").unwrap(),
                        usage
                            .split_whitespace()
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>(),
                    )
                })
                .map(|((x, y), z)| ((x.parse().unwrap(), y.parse().unwrap()), z))
                .collect::<Vec<_>>(),
        }
    }

    fn one(&self) {
        let block_count = self
            .blocks
            .iter()
            .map(|block| block.iter().filter(|&bit| *bit).count())
            .collect::<Vec<usize>>();
        println!(
            "Puzzle 12.1: {}",
            self.regions
                .iter()
                .filter(|((x, y), z)| x * y
                    >= z.iter().enumerate().map(|(i, n)| n * block_count[i]).sum())
                .count()
        );
    }
}
