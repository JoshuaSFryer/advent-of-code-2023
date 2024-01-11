use std::fs;

// Structure to track the greatest number of each colour of cubes in a game
// struct foobar

#[derive(Debug)]
struct Game {
    id: u32,
    maxima: ColourCount,
}

#[derive(Debug)]
struct ColourCount {
    red: u32,
    green: u32,
    blue: u32,
}

fn get_count_from_elem(elem: &str) -> u32 {
    let digits = elem.matches(char::is_numeric);
    let mut sum = 0;

    for (place_val, digit) in digits.rev().enumerate() {
        let base = digit.parse::<u32>().unwrap();
        let exponent = u32::try_from(place_val).unwrap();
        let scalar = 10_u32.pow(exponent);
        let full_number = base * scalar;
        sum += full_number;
    }
    return sum;
}

fn parse_draws(draws: Vec<&str>) -> ColourCount {
    let mut maxima = ColourCount {
        red: 0,
        green: 0,
        blue: 0,
    };
    for draw in draws {
        // Split each draw in to constituent colours.
        let draw_elements = draw.split(',');
        let mut draw_counts = ColourCount {
            red: 0,
            green: 0,
            blue: 0,
        };
        // Get the red, green, blue counts.
        for elem in draw_elements {
            if elem.contains("red") {
                draw_counts.red = get_count_from_elem(elem);
            } else if elem.contains("green") {
                draw_counts.green = get_count_from_elem(elem);
            } else if elem.contains("blue") {
                draw_counts.blue = get_count_from_elem(elem);
            }
        }

        // If this draw showed more of a colour than we've seen so far, update
        // the maximums.
        if draw_counts.red > maxima.red {
            maxima.red = draw_counts.red;
        }
        if draw_counts.green > maxima.green {
            maxima.green = draw_counts.green;
        }
        if draw_counts.blue > maxima.blue {
            maxima.blue = draw_counts.blue;
        }
    }
    return maxima;
}

fn main() {
    let file_path = "./src/input.txt";
    println!("Reading from file {}", file_path);

    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    let contents: String = fs::read_to_string(file_path).expect("Unable to read input file");
    let mut games_list: Vec<Game> = vec![];
    // Each line has several draws, each delineated by a semicolon
    // Each is also identified by an ID, "Game N: {...}"
    for line in contents.lines() {
        // Split the line into Game ID and the list of draws.
        let mut split_line = line.split(':');
        let game_id: &str = split_line.next().unwrap().split(' ').last().unwrap();
        let game_id: u32 = game_id.parse().unwrap();
        let draws: Vec<&str> = split_line.next().unwrap().split(';').collect();
        let game_maxima = parse_draws(draws);
        let curr_game = Game {
            id: game_id,
            maxima: game_maxima,
        };
        games_list.push(curr_game);
    }

    let mut valid_games: Vec<Game> = vec![];
    for game in games_list {
        // Check whether the game is valid; i.e. whether none of its draws had
        // any colour count exceeding the MAX_*** limits.
        if game.maxima.red <= MAX_RED
            && game.maxima.green <= MAX_GREEN
            && game.maxima.blue <= MAX_BLUE
        {
            valid_games.push(game);
        }
    }
    println!("{:?}", valid_games);
    // Sum the IDs of valid games
    let mut sum_of_ids = 0;
    for game in valid_games {
        sum_of_ids += game.id;
    }

    println!("The sum of all valid game IDs is {sum_of_ids}");
}
