use std::collections::HashMap;
use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;

fn replace_card(cards: &mut Vec<part1::Cards>, card: part1::Cards, new_card: part1::Cards) {
    for i in 0..cards.len() {
        if cards[i] == card {
            cards[i] = new_card;
        }
    }
}

fn replace_jokers(cards: &Vec<part1::Cards>) -> Vec<part1::Cards> {
    let mut cards: Vec<part1::Cards> = cards.clone();

    let num_jokers: usize = cards
        .iter()
        .filter(|&card| *card == part1::Cards::CJoker)
        .count();

    if num_jokers > 0 {
        let cards_without_jokers: Vec<part1::Cards> = cards
            .iter()
            .copied()
            .filter(|&card| card != part1::Cards::CJoker)
            .collect();

        let mut cards_freq: HashMap<part1::Cards, usize> = HashMap::new();
        for card in &cards_without_jokers {
            *cards_freq.entry(*card).or_insert(0) += 1;
        }

        if cards_freq.len() == 1 {
            // First case: all cards are the same, let the joker be the first card
            replace_card(&mut cards, part1::Cards::CJoker, cards_without_jokers[0]);
        } else {
            // Second case: Some cards are the same, let the joker be the same as the most frequent card
            // If there are multiple most frequent cards, let the joker be the highest card
            let mut max_freq: usize = 0;
            let mut max_freq_card: part1::Cards = part1::Cards::CJoker;
            let mut higest_card: part1::Cards = part1::Cards::CJoker;
            for (card, freq) in cards_freq {
                if freq > max_freq {
                    max_freq = freq;
                    max_freq_card = card;
                    higest_card = card;
                } else if freq == max_freq {
                    if card > higest_card {
                        higest_card = card;
                    }
                }
            }

            replace_card(&mut cards, part1::Cards::CJoker, max_freq_card);
        }
    }

    return cards;
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let data: Vec<(&str, i64)> = part1::process_input(&input);
    let mut hands: Vec<part1::Hand> = Vec::new();
    for i in 0..data.len() {
        let (cards_str, bid): (&str, i64) = data[i];
        let cards: Vec<part1::Cards> = part1::card_str_to_vec(cards_str, true);
        let cards_without_jokers: Vec<part1::Cards> = replace_jokers(&cards);
        let strength: part1::HandStrength = part1::Hand::compute_strength(&cards_without_jokers);
        let hand: part1::Hand = part1::Hand {
            cards: cards,
            strength: strength,
            bid: bid,
        };
        hands.push(hand);
    }
    let bid: i64 = part1::get_bid(hands);
    return bid.to_string();
}

fn main() {
    // Get input from txt file content
    let input_path: &str = &get_full_path("src/bin/part2/input.txt");
    let input: String = get_file_content(&input_path);
    if input.is_empty() {
        panic!("Input is empty!");
    }

    // Process data
    let output: String = advent_of_code(&input);
    println!("Result: {}", output);

    // Write result to file
    let output_path: &str = &get_full_path("src/bin/part2/output.txt");
    write_file_content(&output_path, &output);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::{get_full_path, test_advent_of_code};

    #[test]
    fn check_code() {
        let input_test_path: &str = &get_full_path("src/bin/part2/input_test.txt");
        let output_test_path: &str = &get_full_path("src/bin/part2/output_test.txt");

        assert_eq!(
            test_advent_of_code(input_test_path, output_test_path, advent_of_code),
            true
        );
    }
}
