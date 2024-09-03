// #![allow(unused_variables)]
// #![allow(unused_mut)]

pub fn solve() {
    
    let file_path = "input.txt";

    let contents = std::fs::read_to_string(file_path)
        .expect("No input file");

    one(&contents);
    two(&contents);

}

fn one(contents: &String) {
    // Max values for each color
    let red = 12;
    let green = 13;
    let blue = 14;

    println!("{}", contents);

    let mut id = 0;
    let mut total = 0;

    for line in contents.lines() {
        // change to correct game ID
        id += 1;
        let mut valid = true;
        let game = line.split(":").collect::<Vec<&str>>()[1];
        for set in game.split(";") {
            for color in set.split(",") {
                let split = color.split(" ").collect::<Vec<&str>>();
                println!("{:?}", split);
                let (count, color) = (split[1].parse::<i32>().unwrap(), split[2]);
                match color {
                    "red" => {
                        if count > red { valid = false; }
                    }
                    "green" => {
                        if count > green { valid = false; }
                    }
                    "blue" => {
                        if count > blue { valid = false; }
                    }
                    _ => { break }
                }
                if !valid { break; }
            }
            if !valid { break; }
        }
        if valid {
            total += id;
        }
    }
    println!("Total: {}", total);
}

fn two(contents: &String) {
    let mut total = 0;

    for line in contents.lines() {

        let mut red: u32 = 0;
        let mut blue: u32 = 0;
        let mut green: u32 = 0;

        let game = line.split(":").collect::<Vec<&str>>()[1];
        for set in game.split(";") {
            for color in set.split(",") {
                let split = color.split(" ").collect::<Vec<&str>>();
                let (count, color) = (split[1].parse::<u32>().unwrap(), split[2]);
                match color {
                    "red" => {
                        if count > red { red = count }
                    }
                    "blue" => {
                        if count > blue { blue = count }
                    }
                    "green" => {
                        if count > green { green = count }
                    }
                    _ => {
                        break;
                    }
                }
            }
        }

        total += red * blue * green
        
    }
    println!("{}", total)
}