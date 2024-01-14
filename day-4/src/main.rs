use std::fs;

struct Card {
    id: u32,
    winning_nums: Vec<u32>,
    your_nums: Vec<u32>,
}

impl Card {
    fn compute_score(&self) -> u32 {
        let mut score = 0;
        for num in &self.your_nums {
            if self.winning_nums.contains(&num) {
                match score {
                    0 => score = 1,
                    _ => score = score * 2,
                }
            }
        }
        score
    }
}

fn assemble_digits_to_number(digits: &Vec<u32>) -> u32 {
    let mut number: u32 = 0;
    for (place_val, digit) in digits.iter().rev().enumerate() {
        number += digit * (10 as u32).pow(place_val as u32);
    }
    number
}

fn parse_card_id(line: &str) -> u32 {
    let header = &line.split(':').next().unwrap();
    let parsed_header = header.matches(|d: char| d.is_ascii_digit());
    let digits_found: Vec<u32> = parsed_header
        .map(|mtch| mtch.parse::<u32>().unwrap())
        .collect();
    let id = assemble_digits_to_number(&digits_found);
    id
}

fn parse_card_numbers(line: &str) -> (Vec<u32>, Vec<u32>) {
    let mut body = line.split(':').last().unwrap().split('|');
    let winning_nums: Vec<u32> = body
        .next()
        .unwrap()
        .split_whitespace()
        .map(|d: &str| d.parse::<u32>().unwrap())
        .collect();
    let player_nums: Vec<u32> = body
        .next()
        .unwrap()
        .split_whitespace()
        .map(|d: &str| d.parse::<u32>().unwrap())
        .collect();

    (winning_nums, player_nums)
}

fn parse_cards(contents: String) -> Vec<Card> {
    let mut cards: Vec<Card> = vec![];
    for line in contents.lines() {
        let id = parse_card_id(line);
        let (winning_nums, player_nums) = parse_card_numbers(line);
        cards.push(Card {
            id: id,
            winning_nums: winning_nums,
            your_nums: player_nums,
        })
    }
    cards
}

fn main() {
    let file_path = "./src/input.txt";
    println!("Reading from file {}", file_path);
    let contents: String = fs::read_to_string(file_path).expect("Unable to read input file");
    let cards = parse_cards(contents);
    let mut score_sum = 0;
    for card in cards {
        let score = card.compute_score();
        let id = card.id;
        println!("Card {id}: score = {score}");
        score_sum += score;
    }
    println!("Total score of all cards: {score_sum}");
}
