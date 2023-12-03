// use std::env;
use test_utils::{get_file_content, get_full_path, write_file_content};

pub fn sum_first_and_last_digits(line: &str) -> u32 {
    let first_digit: u32 = line.chars().next().unwrap().to_digit(10).unwrap_or(0);
    let last_digit: u32 = line.chars().last().unwrap().to_digit(10).unwrap_or(0);
    return first_digit * 10 + last_digit;
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let mut input_numeric: String = String::from("");

    // Remove all non-numeric characters
    for line in input.lines() {
        for char in line.chars() {
            if char.is_numeric() {
                input_numeric.push(char);
            }
        }
        input_numeric.push('\n');
    }

    // For each line, get first and last digit and convert into a number. Sum all numbers.
    let mut sum: u32 = 0;

    for line in input_numeric.lines() {
        sum += sum_first_and_last_digits(&line);
    }

    return sum.to_string();
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
