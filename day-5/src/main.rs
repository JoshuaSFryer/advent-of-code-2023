use std::collections::HashMap;
use std::fs;

// enum Headers {
//     Seeds,
//     SeedSoil,
//     SoilFert,
//     FertWater,
//     WaterLight,
//     LightTemp,
//     TempHumid,
//     HumidLocation,
// }

fn parse_input_file(input: String) {
    const HEADERS: [&str; 8] = [
        "seeds",
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];
    for line in input.lines() {
        println!("{line}");
        let words = line.split(" ");
        // Check whether the line is a header
    }
}

struct Mapping {
    dest_start: u32,
    src_start: u32,
    range_len: u32,
}

fn main() {
    let file_path = "./src/input.txt";
    println!("Reading from file {}", file_path);
    let contents: String = fs::read_to_string(file_path).expect("Unable to read input file");
    parse_input_file(contents);
}
