use std::collections::HashMap;
use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;
use part1::{count_valid_arrangements, process_input};

fn advent_of_code(input_text: &str) -> String {
    let mut valid_arrangements_sum = 0;

    let input: String = String::from(input_text);
    let springs_map = process_input(&input, 5);

    for sm in springs_map {
        let mut cache = &mut HashMap::new();
        let groups = sm.groups.clone();
        let valid_arrangements = count_valid_arrangements(&sm.cfg, &groups, &mut cache);
        valid_arrangements_sum += valid_arrangements;
    }
    return valid_arrangements_sum.to_string();
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
