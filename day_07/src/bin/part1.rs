use std::collections::HashMap;
use test_utils::{get_file_content, get_full_path, write_file_content};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Cards {
    CJoker = 0,
    C2 = 1,
    C3 = 2,
    C4 = 3,
    C5 = 4,
    C6 = 5,
    C7 = 6,
    C8 = 7,
    C9 = 8,
    CT = 9,
    CJ = 10,
    CQ = 11,
    CK = 12,
    CA = 13,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandStrength {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Cards>,
    pub strength: HandStrength,
    pub bid: i64,
}

impl Hand {
    pub fn new(cards: &Vec<Cards>, bid: i64) -> Hand {
        let strength: HandStrength = Hand::compute_strength(&cards.clone());
        let cards: Vec<Cards> = cards.clone();
        let hands: Hand = Hand {
            cards: cards,
            strength: strength,
            bid: bid,
        };

        return hands;
    }

    pub fn compute_strength(cards: &Vec<Cards>) -> HandStrength {
        let mut cards_freq: HashMap<Cards, i64> = HashMap::new();
        for &card in cards {
            *cards_freq.entry(card).or_insert(0) += 1;
        }
        let different_cards_count: usize = cards_freq.len();
        let max_card_count: i64 = *cards_freq.values().max().unwrap();

        let hand_strength: HandStrength = match different_cards_count {
            1 => HandStrength::FiveOfAKind,
            2 => {
                if max_card_count == 4 {
                    HandStrength::FourOfAKind
                } else {
                    HandStrength::FullHouse
                }
            }
            3 => {
                if max_card_count == 3 {
                    HandStrength::ThreeOfAKind
                } else {
                    HandStrength::TwoPairs
                }
            }
            4 => HandStrength::Pair,
            _ => HandStrength::HighCard,
        };

        return hand_strength;
    }
}

fn compare_two_hands(hand1: &Hand, hand2: &Hand) -> std::cmp::Ordering {
    if hand1.strength != hand2.strength {
        return hand1.strength.cmp(&hand2.strength);
    } else {
        // Check card by card, which is higher
        for i in 0..hand1.cards.len() {
            let card1: Cards = hand1.cards[i];
            let card2: Cards = hand2.cards[i];
            if card1 != card2 {
                return card1.cmp(&card2);
            }
        }
    }
    return std::cmp::Ordering::Equal;
}

pub fn get_bid(mut hands: Vec<Hand>) -> i64 {
    let mut bid: i64 = 0;
    hands.sort_by(|a, b| compare_two_hands(&a, &b));
    for i in 0..hands.len() {
        let hand: &Hand = &hands[i];
        bid += hand.bid * (i as i64 + 1);
    }
    return bid;
}

pub fn card_str_to_vec(cards_str: &str, use_jokers: bool) -> Vec<Cards> {
    let mut cards: Vec<Cards> = Vec::new();
    for character in cards_str.chars() {
        match character {
            '2' => cards.push(Cards::C2),
            '3' => cards.push(Cards::C3),
            '4' => cards.push(Cards::C4),
            '5' => cards.push(Cards::C5),
            '6' => cards.push(Cards::C6),
            '7' => cards.push(Cards::C7),
            '8' => cards.push(Cards::C8),
            '9' => cards.push(Cards::C9),
            'T' => cards.push(Cards::CT),
            'J' => {
                if use_jokers {
                    cards.push(Cards::CJoker)
                } else {
                    cards.push(Cards::CJ)
                }
            }
            'Q' => cards.push(Cards::CQ),
            'K' => cards.push(Cards::CK),
            'A' => cards.push(Cards::CA),
            _ => panic!("Unknown card!"),
        }
    }
    return cards;
}

pub fn process_input(input: &str) -> Vec<(&str, i64)> {
    let mut lines: Vec<(&str, i64)> = Vec::new();

    for line in input.lines() {
        let line_values: Vec<&str> = line.split_whitespace().collect();
        let cards_str: &str = line_values[0];
        let bid: i64 = line_values[1].parse::<i64>().unwrap();

        let data: (&str, i64) = (cards_str, bid);
        lines.push(data);
    }

    return lines;
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let data: Vec<(&str, i64)> = process_input(&input);
    let mut hands: Vec<Hand> = Vec::new();
    for i in 0..data.len() {
        let (cards_str, bid): (&str, i64) = data[i];
        let cards: Vec<Cards> = card_str_to_vec(cards_str, false);
        let hand: Hand = Hand::new(&cards, bid);
        hands.push(hand);
    }
    let bid: i64 = get_bid(hands);
    return bid.to_string();
}

#[cfg_attr(not(test), allow(unused))]
fn main() {
    // Get input from txt file content
    let input_path: &str = &get_full_path("src/bin/part1/input.txt");
    let input: String = get_file_content(&input_path);
    if input.is_empty() {
        panic!("Input is empty!");
    }

    // Process data
    let output: String = advent_of_code(&input);
    println!("Result: {}", output);

    // Write result to file
    let output_path: &str = &get_full_path("src/bin/part1/output.txt");
    write_file_content(&output_path, &output);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::{get_full_path, test_advent_of_code};

    #[test]
    fn check_code() {
        let input_test_path: &str = &get_full_path("src/bin/part1/input_test.txt");
        let output_test_path: &str = &get_full_path("src/bin/part1/output_test.txt");
        assert_eq!(
            test_advent_of_code(input_test_path, output_test_path, advent_of_code),
            true
        );
    }
}
