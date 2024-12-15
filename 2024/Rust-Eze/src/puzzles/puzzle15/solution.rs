pub fn solve() {
    let file_path = "src/puzzles/puzzle15/input.txt";

    let contents = std::fs::read_to_string(file_path).expect("No input file");

    one(&contents);
    two(&contents);
}

fn one(contents: &String) {
    let (map, instructions) = contents.split_once("\n\n").unwrap();

    let mut map: Vec<Vec<char>> = map
        .lines()
        .map(|string| string.chars().collect::<Vec<char>>())
        .collect();

    let (mut sy, mut sx): (isize, isize) = map
        .iter()
        .enumerate()
        .filter(|(_y, line)| line.contains(&'@'))
        .map(|(y, line)| {
            (
                y as isize,
                line.iter().position(|t| *t == '@').unwrap() as isize,
            )
        })
        .collect::<Vec<_>>()[0];

    let instructions = instructions.split_whitespace().collect::<String>();

    instructions.chars().for_each(|char| {
        let (mut dx, mut dy): (isize, isize) = (0, 0);
        match char {
            '^' => (dx, dy) = (0, -1),
            '>' => (dx, dy) = (1, 0),
            'v' => (dx, dy) = (0, 1),
            '<' => (dx, dy) = (-1, 0),
            _ => {}
        }

        let (mut nx, mut ny) = (sx + dx, sy + dy);
        let mut pushing = false;
        loop {
            match map[ny as usize][nx as usize] {
                '.' | '@' => {
                    (sx, sy) = (sx + dx, sy + dy);
                    if pushing {
                        map[ny as usize][nx as usize] = 'O';
                        map[sy as usize][sx as usize] = '.'
                    }
                    break;
                }
                'O' => {
                    (nx, ny) = (nx + dx, ny + dy);
                    pushing = true;
                }
                '#' => {
                    break;
                }
                _ => {}
            }
        }
    });

    let mut total = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                total += 100 * y + x;
            }
        }
    }

    println!("Puzzle One: {total}")
}

fn two(contents: &String) {
    let (map, instructions) = contents.split_once("\n\n").unwrap();

    let mut map: Vec<Vec<char>> = map
        .lines()
        .map(|string| {
            string
                .chars()
                .flat_map(|c| {
                    if c == '@' {
                        vec![c, '.']
                    } else if c == 'O' {
                        vec!['[', ']']
                    } else {
                        vec![c, c]
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect();

    let (mut sy, mut sx): (isize, isize) = map
        .iter()
        .enumerate()
        .filter(|(_y, line)| line.contains(&'@'))
        .map(|(y, line)| {
            (
                y as isize,
                line.iter().position(|t| *t == '@').unwrap() as isize,
            )
        })
        .collect::<Vec<_>>()[0];

    map[sy as usize][sx as usize] = '.';
    // println!("{:?}", map);

    // println!("{}, {}", sx, sy);

    let instructions = instructions.split_whitespace().collect::<String>();

    instructions.chars().for_each(|char| {
        let (mut dx, mut dy): (isize, isize) = (0, 0);
        match char {
            '^' => (dx, dy) = (0, -1),
            '>' => (dx, dy) = (1, 0),
            'v' => (dx, dy) = (0, 1),
            '<' => (dx, dy) = (-1, 0),
            _ => {}
        }

        let (nx, ny) = (sx + dx, sy + dy);
        let mut blocks: Vec<(isize, isize)> = Vec::new();
        if pushable(sx, sy, dx, dy, &map, &mut blocks) {
            (sx, sy) = (nx, ny);
            if blocks.len() > 0 {
                if dy != 0 {
                    if dy == -1 {
                        blocks.sort_by(|(_a, b), (_c, d)| b.cmp(d));
                    } else {
                        blocks.sort_by(|(_a, b), (_c, d)| d.cmp(b));
                    }
                    for (x, y) in blocks {
                        let current = map[y as usize][x as usize];
                        map[(y + dy) as usize][(x + dx) as usize] = current;
                        map[y as usize][x as usize] = '.';
                    }
                } else {
                    blocks.reverse();
                    for (x, y) in blocks {
                        let current = map[y as usize][x as usize];
                        map[(y + dy) as usize][(x + dx) as usize] = current;
                        map[y as usize][x as usize] = '.';
                    }
                }
            }
        }
    });

    let total = map
        .iter()
        .enumerate()
        .map(|(y, vec)| {
            (y,
                vec
                    .iter()
                    .enumerate()
                    .filter(|(_x, &value)| value == '[')
                    .map(|(x, _value)| x)
                    .collect::<Vec<_>>())
        })
        .filter(|(_y, x)| x.len() != 0)
        .map(|(y, x)| y*100*x.len() + x.iter().sum::<usize>())
        .sum::<usize>();

    println!("Puzzle Two: {total}")
}

fn pushable(
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    map: &Vec<Vec<char>>,
    blocks: &mut Vec<(isize, isize)>,
) -> bool {
    let (nx, ny) = (x + dx, y + dy);
    match map[ny as usize][nx as usize] {
        '[' => {
            if !blocks.contains(&(nx, ny)) {
                blocks.push((nx, ny));
            }
            if dy != 0 {
                if !blocks.contains(&(nx + 1, ny)) {
                    blocks.push((nx + 1, ny));
                }
                pushable(nx + 1, ny, dx, dy, map, blocks) && pushable(nx, ny, dx, dy, map, blocks)
            } else {
                pushable(nx, ny, dx, dy, map, blocks)
            }
        }
        ']' => {
            if !blocks.contains(&(nx, ny)) {
                blocks.push((nx, ny));
            }
            if dy != 0 {
                if !blocks.contains(&(nx - 1, ny)) {
                    blocks.push((nx - 1, ny));
                }
                pushable(nx - 1, ny, dx, dy, map, blocks) && pushable(nx, ny, dx, dy, map, blocks)
            } else {
                pushable(nx, ny, dx, dy, map, blocks)
            }
        }
        '#' => false,
        _ => true,
    }
}
