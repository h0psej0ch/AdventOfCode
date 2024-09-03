use std::collections::HashMap;

pub fn solve() {
    let file_path = "src/puzzles/puzzle17/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let lines: Vec<Vec<_>> = contents
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let height: usize = lines.len() as usize;
    let length: usize = lines[0].len() as usize;

    // Structure: (Y, X, <ParentNode:(Y, X), direction used) => (Cost, <last direction>:(dy, dx), <parent>:(Y, X))
    let mut unvisited: HashMap<
        (usize, usize, (usize, usize), usize),
        (u32, (i32, i32), (usize, usize)),
    > = HashMap::new();

    // Structure: (Y, X, <ParentNode:(Y, X), direction used) => Cost
    let mut visited: HashMap<(usize, usize, (usize, usize), usize), u32> = HashMap::new();

    let height = height as i32;
    let length = length as i32;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    visited.insert((0, 0, (0, 0), 0), lines[0][0]);
    println!("{}", lines[0][0] + lines[1][0]);
    unvisited.insert(
        (1, 0, (0, 0), 1),
        (lines[0][0] + lines[1][0], (1, 0), (0, 0)),
    );
    println!("{}", lines[0][0] + lines[0][1]);
    unvisited.insert(
        (0, 1, (0, 0), 1),
        (lines[0][0] + lines[0][1], (0, 1), (0, 0)),
    );

    while unvisited.len() > 0 {
        if let Some((&node, &value)) = unvisited.iter().min_by_key(|&(_, &v)| v.0) {
            if value.0 != u32::MAX {
                println!(
                    "Node with the lowest value: {:?} with value: {:?}",
                    node, value
                );
            }
            for direction in directions {
                if direction != opposite_direction(value.1)
                    && !(direction == value.1 && node.3 == 3)
                {
                    let coordinate: (i32, i32) =
                        (node.0 as i32 + direction.0, node.1 as i32 + direction.1);
                    if coordinate.0 >= 0
                        && coordinate.1 >= 0
                        && coordinate.0 < length
                        && coordinate.1 < height
                    {
                        println!("{:?}", coordinate);
                        let con;
                        if direction == value.1 {
                            con = node.3 + 1;
                        } else {
                            con = 1;
                        }

                        let coordinate = (
                            coordinate.0 as usize,
                            coordinate.1 as usize,
                            (node.0, node.1),
                            con,
                        );

                        let new_value = value.0 + lines[coordinate.0][coordinate.1];
                        println!("{}", new_value);

                        if unvisited.contains_key(&coordinate)
                            && (new_value < unvisited.get(&coordinate).unwrap().0)
                        // || (new_value == unvisited.get(&coordinate).unwrap().0
                        //     && con < unvisited.get(&coordinate).unwrap().1))
                        {
                            unvisited.insert(coordinate, (new_value, direction, (node.0, node.1)));
                        } else if !unvisited.contains_key(&coordinate)
                            && !visited.contains_key(&coordinate)
                        {
                            unvisited.insert(coordinate, (new_value, direction, (node.0, node.1)));
                        }
                    }
                }
            }
            unvisited.remove(&node);
            visited.insert(node, value.0);
        }
    }

    // PRINTING THE ANSWER!!!

    let x_max = length as usize - 1;
    let y_max = height as usize - 1;

    // Collect entries into a vector
    let entries: Vec<_> = visited.iter().collect();

    // Filter entries to include only those with x == x_max and y == y_max
    let mut filtered_entries: Vec<_> = entries
        .into_iter()
        .filter(|((y, x, _, _), _)| *x == x_max && *y == y_max)
        .collect();

    // Sort the filtered vector by Y, X, and then by Parent (Y, X)
    filtered_entries.sort_by(|a, b| {
        let ((y1, x1, (py1, px1), _), _) = a;
        let ((y2, x2, (py2, px2), _), _) = b;
        y1.cmp(y2)
            .then_with(|| x1.cmp(x2))
            .then_with(|| py1.cmp(py2))
            .then_with(|| px1.cmp(px2))
    });

    // Print the filtered and sorted entries
    println!(
        "{:<10} {:<10} {:<15} {:<10} {:<10}",
        "Y", "X", "Parent (Y, X)", "Direction", "Cost"
    );
    for ((y, x, (py, px), direction), cost) in filtered_entries {
        println!(
            "{:<10} {:<10} ({:<3}, {:<3}) {:<10} {:<10}",
            y, x, py, px, direction, cost
        );
    }
}

fn opposite_direction(direction: (i32, i32)) -> (i32, i32) {
    (-direction.0, -direction.1)
}

fn two(contents: &String) {
    let lines: Vec<Vec<_>> = contents
        .split("\n")
        .map(|line| {
            line.chars()
                .map(|character| character.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let height: i32 = lines.len() as i32;
    let length: i32 = lines[0].len() as i32;

    // Structure: (Y, X, <Direction:(Y, X)) => Cost)
    let mut unvisited: HashMap<(usize, usize, (i32, i32)), u32> = HashMap::new();

    // Structure: (Y, X, <Direction:(Y, X)) => Cost
    let mut visited: HashMap<(usize, usize, (i32, i32)), u32> = HashMap::new();

    visited.insert((0, 0, (0, 0)), lines[0][0]);

    let mut temp_y = 0;
    let mut temp_x = 0;

    for i in 1..4 {
        temp_y += lines[i][0];
        temp_x += lines[0][i]
    }

    println!("{}, {}", temp_y, temp_x);

    for i in 4..11 {
        if i < length as usize {
            temp_x += lines[0][i];
            unvisited.insert((0, i, (0, 1)), temp_x);
        }
        if i < height as usize {
            temp_y += lines[i][0];
            unvisited.insert((i, 0, (1, 0)), temp_y);
        }
    }

    println!("{:?}", unvisited);

    let mut directions: [(i32, i32); 2];

    while unvisited.len() > 0 {
        if let Some((&node, &value)) = unvisited.iter().min_by_key(|&(_, &v)| v) {
            if node.2 == (0, 1) || node.2 == (0, -1) {
                directions = [(1, 0), (-1, 0)]
            } else {
                directions = [(0, 1), (0, -1)]
            }

            for direction in directions {
                let mut combined_cost = value;
                for i in 1..11 {
                    let new_coords = (node.0 as i32 + i * direction.0, node.1 as i32 + i * direction.1);
                    if new_coords.0 >= 0
                        && new_coords.1 >= 0
                        && new_coords.0 < height
                        && new_coords.1 < length
                    {
                        let new_coords = (new_coords.0 as usize, new_coords.1 as usize);
                        combined_cost += lines[new_coords.0][new_coords.1];
                        let key = (new_coords.0, new_coords.1, direction);
                        if i >= 4 {
                            if (unvisited.contains_key(&key)
                                && unvisited.get(&key).unwrap() > &combined_cost)
                                || !unvisited.contains_key(&key) && !visited.contains_key(&key)
                            {
                                unvisited.insert(key, combined_cost);
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
            unvisited.remove(&node);
            visited.insert(node, value);
        }
    }

    // PRINTING THE ANSWER!!!

    let x_max = length as usize - 1;
    let y_max = height as usize - 1;

    // Collect entries into a vector
    let entries: Vec<_> = visited.iter().collect();

    // Filter entries to include only those with x == x_max and y == y_max
    let mut filtered_entries: Vec<_> = entries
        .into_iter()
        .filter(|((y, x, _), _)| *x == x_max && *y == y_max)
        .collect();

    // Sort the filtered vector by Y, X, and then by Parent (Y, X)
    filtered_entries.sort_by(|a, b| {
        let ((y1, x1, (py1, px1)), _) = a;
        let ((y2, x2, (py2, px2)), _) = b;
        y1.cmp(y2)
            .then_with(|| x1.cmp(x2))
            .then_with(|| py1.cmp(py2))
            .then_with(|| px1.cmp(px2))
    });

    // Print the filtered and sorted entries
    println!(
        "{:<10} {:<10} {:<15} {:<10} {:<10}",
        "Y", "X", "Parent (Y, X)", "Direction", "Cost"
    );
    for ((y, x, (py, px)), cost) in filtered_entries {
        println!(
            "{:<10} {:<10} ({:<3}, {:<3}) {:<10}",
            y, x, py, px, cost
        );
    }

}
