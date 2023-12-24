use lib_utils::Matrix;
use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;
use part1::process_input;

fn compare_pseudo_rows(pattern: &Matrix<u8>, row1: usize, row2: usize, max_num_diff: u8) -> bool {
    let mut num_diff = 0;
    for col in 0..pattern.cols {
        if pattern.at(row1, col) != pattern.at(row2, col) {
            num_diff += 1;
            if num_diff > max_num_diff {
                return false;
            }
        }
    }
    true
}

fn find_simetry(pattern: &Matrix<u8>, row1: usize, row2: usize, max_num_diff: u8) -> bool {
    if row1 >= pattern.rows || row2 >= pattern.rows || row1 >= row2 {
        return false;
    }

    let simetry = compare_pseudo_rows(pattern, row1, row2, max_num_diff);
    if row1 == 0 || row2 == pattern.rows - 1 || !simetry {
        return simetry;
    }
    let next_row1 = row1 - 1;
    let next_row2 = row2 + 1;
    return find_simetry(pattern, next_row1, next_row2, max_num_diff);
}

fn find_mirrors(pattern: &Matrix<u8>, max_num_diff: u8) -> Vec<u64> {
    let mut mirrows: Vec<u64> = Vec::new();
    for row1 in 1..pattern.rows {
        if find_simetry(pattern, row1 - 1, row1, max_num_diff) {
            mirrows.push(row1 as u64);
        }
    }
    return mirrows;
}

fn advent_of_code(input_text: &str) -> String {
    let mut result = 0;
    let valley: Vec<Matrix<u8>> = process_input(input_text);
    for pattern in &valley {
        let count_rows = part1::find_mirrors(pattern);
        let pseudo_count_rows = find_mirrors(pattern, 1);
        let transposed_pattern = pattern.transpose();
        let count_cols = part1::find_mirrors(&transposed_pattern);
        let pseudo_count_cols = find_mirrors(&transposed_pattern, 1);

        let mut pseudo_count_rows = pseudo_count_rows
            .into_iter()
            .filter(|x| !count_rows.contains(x))
            .collect::<Vec<u64>>();
        let mut pseudo_count_cols = pseudo_count_cols
            .into_iter()
            .filter(|x| !count_cols.contains(x))
            .collect::<Vec<u64>>();

        for row in pseudo_count_rows {
            result += 100 * row;
        }
        for col in pseudo_count_cols {
            result += col;
        }
    }
    return result.to_string();
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
