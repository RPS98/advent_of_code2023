use test_utils::{get_file_content, get_full_path, write_file_content};

pub fn get_scratchcards_matches(winning_numbers: &Vec<u32>, scratch_result: &Vec<u32>) -> u32 {
    let mut matches: u32 = 0;

    // Get numbers that match in both arrays
    for scratch_number in scratch_result {
        if winning_numbers.contains(scratch_number) {
            matches += 1;
        }
    }
    return matches;
}

pub fn get_scratchcards_numbers(line_text: &str) -> (Vec<u32>, Vec<u32>) {
    // Ej: Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

    // Find indexes
    let start_index = line_text.find(':').unwrap_or(0);
    let pipe_index = line_text.find('|').unwrap_or(0);
    // let card_id: u32 = line_text[5..start_index].parse().unwrap_or(0);
    let winning_numbers_str: &str = &line_text[start_index + 1..pipe_index];
    let scratch_result_str: &str = &line_text[pipe_index + 1..];

    let winning_numbers: Vec<u32> = winning_numbers_str
        .split_whitespace() // Split by whitespace and return an iterator
        .filter_map(|s| s.parse().ok()) // Iterate over the iterator, parse each element, and filter out failures
        .collect(); // Collect the results into a new Vec

    let scratch_result: Vec<u32> = scratch_result_str
        .split_whitespace() // Split by whitespace and return an iterator
        .map(|s| s.parse().unwrap_or(0)) // Iterate over the iterator, parse each element, and filter out failures
        .collect(); // Collect the results into a new Vec

    return (winning_numbers, scratch_result);
}

fn get_card_macthes_points(matches: u32) -> u32 {
    if matches == 0 {
        return 0;
    }
    let points: u32 = 2u32.pow(matches - 1);
    return points;
}

fn advent_of_code(input_text: &str) -> String {
    let mut scratch_points: u32 = 0;

    for line in input_text.lines() {
        let (winning_numbers, scratch_result) = get_scratchcards_numbers(line);
        let matches: u32 = get_scratchcards_matches(&winning_numbers, &scratch_result);
        let card_points: u32 = get_card_macthes_points(matches);
        scratch_points += card_points;
    }
    return scratch_points.to_string();
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
