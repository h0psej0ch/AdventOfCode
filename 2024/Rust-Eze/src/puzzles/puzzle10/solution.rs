use std::collections::HashSet;

pub fn solve() {
    
    let file_path = "src/puzzles/puzzle10/input.txt";

    let contents = std::fs::read_to_string(file_path)
        .expect("No input file");

    one(&contents);
    two(&contents);

}

const DIRECTIONS: &'static [(i32, i32)] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];

fn one(contents: &String) {
    let mut starting_points = Vec::new();
    contents.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if char == '0' {
                starting_points.push((x, y));
            }
        })
    });

    let map = contents.lines().map(|line| line.chars().map(|character| character.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let sum = starting_points.iter().map(|(x, y)| {
        let mut hashset: HashSet<(i32, i32)> = HashSet::new();
        recursion(map.clone(), *x as i32, *y as i32, 0, &mut hashset);
        hashset.len()
    }).sum::<usize>();

    println!("Puzzle One: {}",sum)

}

fn recursion(map: Vec<Vec<u32>>, x: i32, y: i32, current: u32, hash_set: &mut HashSet<(i32, i32)>) {
    if current == 9 {
        hash_set.insert((x, y));
    }
    DIRECTIONS.iter().for_each(|(dx, dy)| {
        let new = (x + dx, y + dy);
        if new.0 >= 0 && new.1 >= 0 && new.0 < map[0].len() as i32 && new.1 < map.len() as i32 && map[(new.1) as usize][(new.0) as usize] == current + 1{
            recursion(map.clone(), x + dx, y + dy, current + 1, hash_set);
        };
    });
}


fn two(contents: &String) {
    let mut starting_points = Vec::new();
    contents.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            if char == '0' {
                starting_points.push((x, y));
            }
        })
    });

    let map = contents.lines().map(|line| line.chars().map(|character| character.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let sum = starting_points.iter().map(|(x, y)| {
        let mut vector = Vec::new();
        recursion_vector(map.clone(), *x as i32, *y as i32, 0, &mut vector);
        vector.len()
    }).sum::<usize>();

    println!("Puzzle Two: {}",sum)
}

fn recursion_vector(map: Vec<Vec<u32>>, x: i32, y: i32, current: u32, vector: &mut Vec<(i32, i32)>) {
    if current == 9 {
        vector.push((x, y));
    }
    DIRECTIONS.iter().for_each(|(dx, dy)| {
        let new = (x + dx, y + dy);
        if new.0 >= 0 && new.1 >= 0 && new.0 < map[0].len() as i32 && new.1 < map.len() as i32 && map[(new.1) as usize][(new.0) as usize] == current + 1{
            recursion_vector(map.clone(), x + dx, y + dy, current + 1, vector);
        };
    });
}