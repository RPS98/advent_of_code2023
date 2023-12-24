use lib_utils::Matrix;
use test_utils::{get_file_content, get_full_path, write_file_content};

pub fn process_input(input: &str) -> Vec<Matrix<u8>> {
    let mut valley: Vec<Matrix<u8>> = Vec::new();

    let input_lines: Vec<&str> = input.lines().collect();
    let mut pattern: Matrix<u8> = Matrix::new(1, input_lines[0].len(), 0);
    let mut first_line_flag = true;

    for line in input_lines {
        if line.len() == 0 {
            first_line_flag = true;
            valley.push(pattern.clone());
            continue;
        }

        if first_line_flag {
            pattern = Matrix::new(1, line.len(), 0);
        }

        let mut values: Vec<u8> = Vec::new();
        for character in line.chars() {
            let value: u8 = match character {
                '.' => 0,
                '#' => 1,
                _ => panic!("Unknown character: {}", character),
            };
            values.push(value);
        }

        pattern.insert_row(pattern.rows, values);

        if first_line_flag {
            pattern.remove_row(0);
            first_line_flag = false;
        }
    }
    valley.push(pattern.clone());
    return valley;
}

fn find_simetry(pattern: &Matrix<u8>, row1: usize, row2: usize) -> bool {
    if row1 >= pattern.rows || row2 >= pattern.rows || row1 >= row2 {
        return false;
    }

    let simetry = pattern.compare_rows(row1, row2);
    if row1 == 0 || row2 == pattern.rows - 1 || !simetry {
        return simetry;
    }
    let next_row1 = row1 - 1;
    let next_row2 = row2 + 1;
    return find_simetry(pattern, next_row1, next_row2);
}

pub fn find_mirrors(pattern: &Matrix<u8>) -> Vec<u64> {
    let mut mirrows: Vec<u64> = Vec::new();
    for row1 in 1..pattern.rows {
        if find_simetry(pattern, row1 - 1, row1) {
            mirrows.push(row1 as u64);
        }
    }
    return mirrows;
}

fn advent_of_code(input_text: &str) -> String {
    let mut result = 0;
    let valley: Vec<Matrix<u8>> = process_input(input_text);
    for pattern in &valley {
        let count_rows = find_mirrors(pattern);
        let transposed_pattern = pattern.transpose();
        let count_cols = find_mirrors(&transposed_pattern);
        for row in count_rows {
            result += 100 * row;
        }
        for col in count_cols {
            result += col;
        }
    }
    return result.to_string();
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
