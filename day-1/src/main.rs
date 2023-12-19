use std::{fs, vec};

fn get_calibration_value(line: &str) -> u32 {
    let numeric_strs: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    // try a closure
    let my_closure = |c: char| -> bool { c.is_digit(10) };

    let digit_matches = line.match_indices(my_closure);
    let mut matches_vector: Vec<(usize, u32)> = vec![];
    for m in digit_matches.clone().into_iter() {
        // Try to convert digit into numeric
        let digit = m.1.parse::<u32>().unwrap();
        let digitized: (usize, u32) = (m.0, digit);
        matches_vector.push(digitized);
    }

    // Get all the instances of spelled-out numbers in the line
    for s in numeric_strs.iter().enumerate() {
        let matching_strings = line.match_indices(*s.1);
        for m in matching_strings.into_iter() {
            // Convert "one", "two", etc into digit
            let digit: u32 = (s.0 + 1).try_into().unwrap();
            matches_vector.push((m.0, digit));
        }
    }

    // Sort the pooled matches
    matches_vector.sort_by(|a, b| a.0.cmp(&b.0));

    let first_match = matches_vector.first();
    let last_match = matches_vector.last();

    let first_val = first_match.unwrap().1;
    let last_val = match last_match {
        Some(num) => num.1,
        None => first_val,
    };

    return (10 * first_val) + last_val;
}

fn main() {
    let file_path = "./src/input.txt";
    println!("Reading from file {}", file_path);

    let contents: String = fs::read_to_string(file_path).expect("Unable to read input file");

    let mut sum = 0;
    for line in contents.lines() {
        sum += get_calibration_value(line);
    }

    println!("Final sum: {sum}");
}
