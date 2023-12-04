use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;

fn advent_of_code(input_text: &str) -> String {
    let mut cards_gains: Vec<Vec<u32>> = vec![vec![]; input_text.lines().count()];

    for i in 0..input_text.lines().count() {
        let line: &str = input_text.lines().nth(i).unwrap();
        let (winning_numbers, scratch_result) = part1::get_scratchcards_numbers(line);
        let matches: u32 = part1::get_scratchcards_matches(&winning_numbers, &scratch_result);

        for j in 0..matches {
            cards_gains[i].push(i as u32 + j + 1);
        }
    }

    // Cards gains, for each index, the cards indexes that are gained
    // 1: [2, 3, 4, 5]
    // 2: [3, 4]
    // 3: [4, 5]
    // 4: [5]
    // 5: []
    // 6: []

    // Cards count, for each index, the cards count that are gained
    // 6: 1
    // 5: 1
    // 4: 1 + index_5 = 1 + 1 = 2
    // 3: 1 + index_4 + index_5 = 1 + 2 + 1 = 4
    // 2: 1 + index_3 + index_4 = 1 + 4 + 2 = 7
    // 1: ...

    let mut cards_count: Vec<u32> = vec![1; input_text.lines().count()];
    for i in 0..cards_gains.len() {
        let idx: usize = cards_gains.len() - i - 1;

        for j in 0..cards_gains[idx].len() {
            let index = cards_gains[idx][j] as usize;
            cards_count[idx] += cards_count[index]
        }
    }

    let cards_sum: u32 = cards_count.iter().sum();
    return cards_sum.to_string();
}

#[cfg_attr(not(test), allow(unused))]
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
