use regex::Regex;

pub fn solve() {
    let file_path = "src/puzzles/puzzle13/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();
    let machines = contents
        .split("\n\n")
        .map(|machine_data| machine_data.lines().collect())
        .map(|machine_vector: Vec<_>| {
            let capture_a = parse_capture(&re.captures(machine_vector[0]).unwrap());
            let capture_b = parse_capture(&re.captures(machine_vector[1]).unwrap());
            let capture_prize = parse_capture(&re.captures(machine_vector[2]).unwrap());

            let matrix = [
                [capture_a.0, capture_b.0],
                [capture_a.1, capture_b.1],
            ];

            inverse(matrix, capture_prize)
        })
        .filter_map(|result| result)
        .map(|(x, y)| x * 3 + 1 * y)
        .sum::<isize>();

    println!("Puzzle One: {}", machines)
}

fn parse_capture(capture: &regex::Captures) -> (isize, isize) {
    (
        capture.get(1).unwrap().as_str().parse::<isize>().unwrap(),
        capture.get(2).unwrap().as_str().parse::<isize>().unwrap(),
    )
}

fn inverse(matrix: [[isize; 2]; 2], vector: (isize, isize)) -> Option<(isize, isize)> {
    let det = matrix[0][0] * matrix [1][1] - matrix[0][1] * matrix[1][0];
    if det == 0 {
        return None
    }
    let x = vector.0 * matrix[1][1] - vector.1 * matrix[0][1];
    let y = vector.1 * matrix[0][0] - vector.0 * matrix[1][0];
    if x % det == 0 && y % det == 0 {
        Some((x/det, y/det))
    } else {
        None
    }
}

fn two(contents: &String) {
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();
    let machines = contents
        .split("\n\n")
        .map(|machine_data| machine_data.lines().collect())
        .map(|machine_vector: Vec<_>| {
            let capture_a = parse_capture(&re.captures(machine_vector[0]).unwrap());
            let capture_b = parse_capture(&re.captures(machine_vector[1]).unwrap());
            let capture_prize = parse_capture(&re.captures(machine_vector[2]).unwrap());

            let matrix = [
                [capture_a.0, capture_b.0],
                [capture_a.1, capture_b.1],
            ];

            let capture_prize = (capture_prize.0 + 10000000000000, capture_prize.1 + 10000000000000);

            inverse(matrix, capture_prize)
        })
        .filter_map(|result| result)
        .map(|(x, y)| x * 3 + 1 * y)
        .sum::<isize>();

    println!("Puzzle Two: {}", machines)
}
