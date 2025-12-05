pub fn solve() {
    let file_path = "src/puzzles/puzzle5/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");
    let s = Solver::new(contents);
    s.one();
    s.two();
}

struct Solver {
    fresh: Vec<(u128, u128)>,
    ing: Vec<u128>,
}

impl Solver {
    fn new(contents: String) -> Self {
        let (ranges, ingredients) = contents.split_once("\n\n").unwrap();
        Self {
            fresh: ranges
                .lines()
                .map(|line| line.trim().split_once('-').unwrap())
                .map(|(s1, s2)| (s1.parse::<u128>().unwrap(), s2.parse::<u128>().unwrap()))
                .collect::<Vec<(u128, u128)>>(),
            ing: ingredients
                .lines()
                .map(|line| line.trim().parse::<u128>().unwrap())
                .collect::<Vec<u128>>(),
        }
    }

    fn one(&self) {
        println!(
            "Puzzle 5.1: {}",
            self.ing
                .iter()
                .filter(|&ing| {
                    for (low, high) in &self.fresh {
                        if (ing >= low) && (ing <= high) {
                            return true;
                        }
                    }
                    false
                })
                .count()
        );
    }

    fn two(&self) {
        let mut ranges: Vec<(u128, u128)> = Vec::new();
        self.fresh.iter().for_each(|&(low, high)| {
            ranges = check_range(ranges.clone(), low, high);
        });

        println!(
            "Puzzle 5.2: {}",
            ranges.iter().map(|&(l, h)| h - l + 1).sum::<u128>()
        );
    }
}
fn check_range(mut range: Vec<(u128, u128)>, mut low: u128, mut high: u128) -> Vec<(u128, u128)> {
    let mut skip = false;
    let mut removal: Vec<usize> = Vec::new();
    range.iter().enumerate().for_each(|(n, &(cl, ch))| {
        if low >= cl && low <= ch {
            if high <= ch {
                skip = true;
            } else {
                removal.push(n);
                low = cl;
            }
        } else if high <= ch && high >= cl {
            if low >= cl {
                skip = true;
            } else {
                removal.push(n);
                high = ch;
            }
        } else if high >= ch && low <= cl {
            removal.push(n);
        }
    });
    if !skip {
        removal.reverse();
        removal.iter().for_each(|&n| {
            range.remove(n);
        });
        range.push((low, high));
        range
    } else {
        range
    }
}
