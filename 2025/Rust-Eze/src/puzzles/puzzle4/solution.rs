pub fn solve() {
    let file_path = "src/puzzles/puzzle4/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(_contents: &String) {}

fn two(_contents: &String) {}
