use std::{collections::HashMap, fs};

// Structure to track the greatest number of each colour of cubes in a game
// struct foobar

struct GameMaximums {
    red: u32,
    green: u32,
    blue: u32,
}

// Associate

fn main() {
    let file_path = "./src/input.txt";
    println!("Reading from file {}", file_path);

    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let contents: String = fs::read_to_string(file_path).expect("Unable to read input file");

    // Each line has several draws, each delineated by a semicolon
    // Each is also identified by an ID, "Game N: {...}"
    let mut games_map: HashMap<&str, GameMaximums> = HashMap::new();
    for line in contents.lines() {
        let mut split_line = line.split(':');
        let game_id: &str = split_line.next().unwrap().split(' ').last().unwrap();
        let draws = split_line.next().unwrap().split(';');

        let mut maximums = GameMaximums {
            red: 0,
            green: 0,
            blue: 0,
        };
        for draw in draws {
            // red/green/blue values are split by commas
            let elems = draw.split(',');
            for elem in elems {
                if draw.contains("red") {
                    let red_count = draw
                        .matches(char::is_numeric)
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    if red_count > maximums.red {
                        maximums.red = red_count;
                    }
                } else if draw.contains("green") {
                    let green_count = draw
                        .matches(char::is_numeric)
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    if green_count > maximums.green {
                        maximums.green = green_count;
                    }
                } else if draw.contains("blue") {
                    let blue_count = draw
                        .matches(char::is_numeric)
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap();
                    if blue_count > maximums.blue {
                        maximums.blue = blue_count;
                    }
                }
            }
        }
        println!(".");
        games_map.insert(game_id, maximums);
    }
}
