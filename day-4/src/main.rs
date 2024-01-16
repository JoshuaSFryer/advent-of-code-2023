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

    fn count_matches(&self) -> u32 {
        let mut num_matches = 0;
        for num in &self.your_nums {
            if self.winning_nums.contains(&num) {
                num_matches += 1;
            }
        }
        num_matches
    }
}

// Card rewards never go backwards - i.e. card 14 will never reward a card 13.
// They will also never reward themselves.
// So we can roll forward through the list of cards once, looking forward.
fn payout_copies(card: Card, card_counts: &mut [u32; 203]) -> () {
    // Get the number of matching numbers in current card.
    let num_matches = card.count_matches();

    // Determine which cards are awarded, based on the current card's ID and
    // the number of matches.
    let reward_ids: Vec<u32> = ((card.id + 1)..=(card.id + num_matches)).collect();

    // Add the awarded copies to the set of cards.
    // Scale the number by how many copies of THIS card we have.
    // All copies of a card will award identically, so we can do this
    // instead of looping.
    let num_copies = card_counts[(card.id - 1) as usize];
    for id in &reward_ids {
        card_counts[(id - 1) as usize] += num_copies;
    }
    let card_id = card.id;
    println!("Card {card_id}: \t {num_copies} copies \t, contains {num_matches} matches.");
    println!("Paying out {num_copies} copies of cards {reward_ids:?}");
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
    let mut cards_sum = 0;
    let mut card_copies_count: [u32; 203] = [1; 203];
    for card in cards {
        let score = card.compute_score();
        let id = card.id;
        let num_matches = card.count_matches();
        let num_copies = card_copies_count[(id - 1) as usize];
        payout_copies(card, &mut card_copies_count);
        // println!("Card {id}: score = {score}");
        // println!("Card {id}: \t {num_copies} copies \t, contains {num_matches} matches.");
        cards_sum += num_copies;
        score_sum += score;
    }
    let other_sum: u32 = card_copies_count.iter().sum();
    println!("Total score of all cards: {score_sum}");
    println!("Total count of all cards: {cards_sum}");
    println!("Total count of all cards: {other_sum}");
}
