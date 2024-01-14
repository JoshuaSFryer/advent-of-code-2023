use std::fs;

const NUM_ROWS: usize = 140;
const NUM_COLS: usize = 140;

const SYMBOL: u32 = 0xC;
const EMPTY: u32 = 0xF;

fn get_neighbours(grid: &Grid, x: usize, y: usize) -> Vec<u32> {
    let mut neighbours = vec![];
    for dy in -1..=1 {
        for dx in -1..=1 {
            // let val = grid.get(x as isize + dx, y as isize + dy);
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x < 0 || new_y < 0 || new_x >= NUM_COLS as isize || new_y >= NUM_ROWS as isize {
                break;
            } else {
                neighbours.push(grid.get(new_x as usize, new_y as usize));
            }
        }
    }
    neighbours
}

#[derive(Clone, Copy, Debug)]
struct Point {
    coords: (u32, u32),
}

#[derive(Debug, Clone, Copy)]
struct PartNumber {
    start: Point,
    length: u32,
    value: u32,
}

struct Grid {
    values: [[u32; NUM_COLS]; NUM_ROWS],
    digit_points: Vec<Point>, // All Points that contain a digit.
    part_numbers: Vec<u32>,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u32 {
        self.values[y as usize][x as usize]
    }
}

fn assemble_digits_to_number(digits: &Vec<u32>) -> u32 {
    let mut number: u32 = 0;
    for (place_val, digit) in digits.iter().rev().enumerate() {
        number += digit * (10 as u32).pow(place_val as u32);
    }
    number
}

fn parse_part_numbers(grid: Grid) -> Vec<PartNumber> {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut number_under_construction: Vec<u32> = vec![];
    let mut constructing_start_point: Point = Point { coords: (0, 0) };
    let mut is_part_number = false;

    for y in 0..NUM_COLS {
        for x in 0..NUM_ROWS {
            let curr_elem = grid.get(x, y);
            // Current element is a digit, 0-9.
            if curr_elem >= 0 && curr_elem <= 9 {
                if number_under_construction.is_empty() {
                    constructing_start_point.coords = (x as u32, y as u32);
                }
                number_under_construction.push(curr_elem);
                // Check for adjacent symbols
                let neighbours = get_neighbours(&grid, x, y);
                let has_adjacent_symbol = neighbours.contains(&SYMBOL);
                if has_adjacent_symbol && !is_part_number {
                    is_part_number = true;
                }

                if x == NUM_COLS - 1 {
                    let finished_num = PartNumber {
                        value: assemble_digits_to_number(&number_under_construction),
                        start: constructing_start_point,
                        length: number_under_construction.len() as u32,
                    };

                    if is_part_number {
                        part_numbers.push(finished_num);
                    }
                    number_under_construction.clear();
                    is_part_number = false;
                }

            // Current element is a non-digit, so end the parsing of the current
            // number.
            } else {
                if !&number_under_construction.is_empty() {
                    // We were building a number, and reached the end of it, so
                    // collect the digits we scanned, assemble them, and save the
                    // resulting number.
                    let finished_num = PartNumber {
                        value: assemble_digits_to_number(&number_under_construction),
                        start: constructing_start_point,
                        length: number_under_construction.len() as u32,
                    };

                    if is_part_number {
                        part_numbers.push(finished_num);
                    }
                    number_under_construction.clear();
                    is_part_number = false;
                }
            }
        }
    }

    part_numbers
}

fn build_grid(file_contents: String) -> Grid {
    let mut vals: [[u32; NUM_COLS]; NUM_ROWS] = [[EMPTY; NUM_COLS]; NUM_ROWS];

    for (y, line) in file_contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                vals[y][x] = c
                    .to_digit(10)
                    .expect("Tried to convert non-digit character to digit.");
            } else if c == '.' {
                vals[y][x] = EMPTY;
            } else {
                vals[y][x] = SYMBOL;
            }
        }
    }

    let grid = Grid {
        values: vals,
        digit_points: vec![],
        part_numbers: vec![],
    };

    grid
}

fn main() {
    let file_path = "./src/input.txt";
    println!("Reading from file {}", file_path);
    let contents: String = fs::read_to_string(file_path).expect("Unable to read input file");
    let grid = build_grid(contents);

    // println!("{(grid.values):?}");
    let vals = &grid.values;
    println!("{vals:?}");

    let part_numbers = parse_part_numbers(grid);
    let mut part_number_sum = 0;
    for num in part_numbers {
        let val = num.value;
        part_number_sum += val;
        println!("{val}");
    }

    println!("Sum of all part numbers: {part_number_sum}");
}
