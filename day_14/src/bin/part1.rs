use lib_utils::Matrix;
use test_utils::{get_file_content, get_full_path, write_file_content};

pub fn process_input(input: &str) -> Matrix<u8> {
    let num_lines: usize = input.lines().count();
    let len_line: usize = input.lines().next().unwrap().len();
    let mut platform: Matrix<u8> = Matrix::new(num_lines, len_line, 0);

    for (row_idx, line) in input.lines().enumerate() {
        for (col_idx, character) in line.chars().enumerate() {
            let value = match character {
                '.' => 0,
                'O' => 1,
                '#' => 2,
                _ => panic!("Unknown character: {}", character),
            };
            platform.set_value(value, row_idx, col_idx);
        }
    }
    platform
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

fn single_move_platform(platform: &mut Matrix<u8>, direction: &Direction) {
    for row_aux in 0..platform.rows {
        for col_aux in 0..platform.cols {
            let row = match direction {
                Direction::North => platform.rows - row_aux - 1,
                _ => row_aux,
            };
            let col = match direction {
                Direction::East => platform.cols - col_aux - 1,
                _ => col_aux,
            };
            let value = *platform.at(row, col);
            if value != 1 {
                continue;
            }
            let mut move_row: usize = row;
            let mut move_col: usize = col;
            match direction {
                Direction::North => {
                    if move_row == 0 {
                        continue;
                    }
                    move_row -= 1;
                }
                Direction::South => {
                    if move_row == platform.rows - 1 {
                        continue;
                    }
                    move_row += 1;
                }
                Direction::East => {
                    if move_col == platform.cols - 1 {
                        continue;
                    }
                    move_col += 1;
                }
                Direction::West => {
                    if move_col == 0 {
                        continue;
                    }
                    move_col -= 1;
                }
            }
            let next_cell_value = *platform.at(move_row, move_col);
            if next_cell_value == 0 {
                platform.set_value(value, move_row, move_col);
                platform.set_value(0, row, col);
            }
        }
    }
}

pub fn move_platform(platform: &mut Matrix<u8>, direction: &Direction) {
    let num_iterations = match direction {
        Direction::North => platform.rows,
        Direction::South => platform.rows,
        Direction::East => platform.cols,
        Direction::West => platform.cols,
    };
    for _ in 0..num_iterations {
        single_move_platform(platform, direction);
    }
}

pub fn compute_load(platform: &Matrix<u8>) -> u64 {
    let mut count_ = 0;

    for row in 0..platform.rows {
        for col in 0..platform.cols {
            let value = *platform.at(row, col);
            if value == 1 {
                count_ += (platform.rows - row) as u64;
            }
        }
    }
    count_
}

fn advent_of_code(input_text: &str) -> String {
    let mut platform: Matrix<u8> = process_input(input_text);
    let direction = Direction::North;
    move_platform(&mut platform, &direction);
    let load = compute_load(&platform);
    return load.to_string();
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
