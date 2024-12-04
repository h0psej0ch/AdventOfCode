pub fn solve() {
    let file_path = "src/puzzles/puzzle4/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let lines = contents.lines();
    let line_array: Vec<_> = lines.clone().collect();

    let char_array: Vec<_> = lines
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let x = char_array.iter().enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate()
            .filter_map(move |(x, &value)| if value == 'X' { Some((y, x)) } else { None })
    });

    let mut count = Vec::new();

    x.for_each(|(y, x)| {
        if x + 3 < line_array[y].len() && &line_array[y][x..=(x + 3)] == "XMAS" {
            count.push((x, y));
        } // Horizontal Forward
        if x >= 3 && &line_array[y][x - 3..=x] == "SAMX" {
            count.push((x, y));
        } // Horizontal Backward

        if y + 3 < line_array.len() && (0..=3).all(|i| line_array[y + i].chars().nth(x) == Some("XMAS".chars().nth(i).unwrap())) {
            count.push((x, y));
        } // Vertical Down
        if y >= 3 && (0..=3).all(|i| line_array[y - i].chars().nth(x) == Some("XMAS".chars().nth(i).unwrap())) {
            count.push((x, y));
        } // Vertical Up

        if y + 3 < line_array.len() && x + 3 < line_array[y].len() && (0..=3).all(|i| line_array[y + i].chars().nth(x + i) == Some("XMAS".chars().nth(i).unwrap())) {
            count.push((x, y));
        } // Diagonal Right Down
        if y >= 3 && x >= 3 && (0..=3).all(|i| line_array[y - i].chars().nth(x - i) == Some("XMAS".chars().nth(i).unwrap())) {
            count.push((x, y));
        } // Diagonal Left Up
        if y >= 3 && x + 3 < line_array[y].len() && (0..=3).all(|i| line_array[y - i].chars().nth(x + i) == Some("XMAS".chars().nth(i).unwrap())) {
            count.push((x, y));
        } // Diagonal Right Up
        if y + 3 < line_array.len() && x >= 3 && (0..=3).all(|i| line_array[y + i].chars().nth(x - i) == Some("XMAS".chars().nth(i).unwrap())) {
            count.push((x, y));
        } // Diagonal Left Down
    });

    println!("Puzzle One: {}", count.len());

    // Visualize marked X
    // for y in 0..line_array.len() {
    //     for x in 0..line_array[y].len() {
    //         if count.contains(&(x, y)) {
    //             print!("X")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!()
    // }

}

fn two(contents: &String) {
    let lines = contents.lines();
    let lines_length = contents.lines().nth(0).unwrap().len();
    let row_length = contents.lines().collect::<Vec<_>>().len();

    let char_array: Vec<_> = lines
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let a = char_array.iter().enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate()
                .filter_map(move |(x, &value)| if value == 'A' && x > 0 && y > 0 && x + 1 < lines_length && y + 1 < row_length { Some((y, x)) } else { None })
        });

    let count = a.filter(|(y, x)| {

        let slope_down =
            (char_array[y - 1][x - 1] == 'M' && char_array[y + 1][x + 1] == 'S') ||
            (char_array[y + 1][x + 1] == 'M' && char_array[y - 1][x - 1] == 'S');

        let slope_up =
            (char_array[y - 1][x + 1] == 'M' && char_array[y + 1][x - 1] == 'S') ||
            (char_array[y + 1][x - 1] == 'M' && char_array[y - 1][x + 1] == 'S');

        slope_down && slope_up
        }
    ).count();

    println!("Puzzle Two: {}", count);
}
