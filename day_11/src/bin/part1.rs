use day_10::Matrix;
use test_utils::{get_file_content, get_full_path, write_file_content};

pub fn expand_universe(space: &mut Matrix<char>) {
    let vec_of_space = vec!['*'; space.cols];
    let space_row_aux = space.rows;
    for i in 0..space_row_aux {
        let row = space_row_aux - i - 1;
        let row_vector = space.get_row_vector(row, 0, space.cols);
        if row_vector.iter().all(|&x| x == '.') {
            space.insert_row(row, vec_of_space.clone());
        }
    }

    let vec_of_space = vec!['*'; space.rows];
    let space_col_aux = space.cols;
    for i in 0..space_col_aux {
        let col = space_col_aux - i - 1;
        let col_vector = space.get_col_vector(col, 0, space.rows);
        if col_vector.iter().all(|&x| x == '.' || x == '*') {
            space.insert_col(col, vec_of_space.clone());
        }
    }
}

pub fn process_input(input: &str) -> Matrix<char> {
    let number_of_lines: usize = input.lines().count();
    let number_of_columns: usize = input.lines().next().unwrap().len();

    let mut space: Matrix<char> = Matrix::new(number_of_lines, number_of_columns, '.');
    for (row, line) in input.lines().rev().enumerate() {
        let row = number_of_lines - row - 1;
        for (col, character) in line.chars().enumerate() {
            if character == '#' {
                space.set_value('#', row, col);
            }
        }
    }
    space
}

pub fn get_galaxies_coordinates(
    space: &Matrix<char>,
    expansion_factor: &usize,
) -> Vec<(usize, usize)> {
    let mut galaxies_coordinates: Vec<(usize, usize)> = Vec::new();
    for row in 0..space.rows {
        for col in 0..space.cols {
            if *space.at(row, col) == '#' {
                // Count how many '*' are between the galaxy and the border

                let horizontal_count_to_border = match col {
                    0 => 0,
                    _ => {
                        let left_part = space.get_row_vector(row, 0, col);
                        left_part.iter().filter(|&x| *x == '*').count()
                    }
                };
                let vertical_count_to_border = match row {
                    0 => 0,
                    _ => {
                        let upper_part = space.get_col_vector(col, 0, row);
                        upper_part.iter().filter(|&x| *x == '*').count()
                    }
                };

                let original_row_coordinate = row - vertical_count_to_border;
                let original_col_coordinate = col - horizontal_count_to_border;

                galaxies_coordinates.push((
                    expansion_factor * vertical_count_to_border + original_row_coordinate,
                    expansion_factor * horizontal_count_to_border + original_col_coordinate,
                ));
            }
        }
    }
    galaxies_coordinates
}

pub fn compute_manhattan_distance_between(row1: i64, col1: i64, row2: i64, col2: i64) -> i64 {
    let row_distance = (row1 - row2).abs();
    let col_distance = (col1 - col2).abs();
    let distance = row_distance + col_distance;
    distance
}

pub fn compute_distance_between_galaxies(space: &Matrix<char>, expansion_factor: &usize) -> i64 {
    let galaxies_coordinates: Vec<(usize, usize)> =
        get_galaxies_coordinates(&space, &expansion_factor);
    let mut distance_sum: i64 = 0;

    for i in 0..galaxies_coordinates.len() {
        for j in (i + 1)..galaxies_coordinates.len() {
            let (row1, col1) = galaxies_coordinates[i];
            let (row2, col2) = galaxies_coordinates[j];
            if row1 == row2 && col1 == col2 {
                continue;
            }
            let distance = compute_manhattan_distance_between(
                row1 as i64,
                col1 as i64,
                row2 as i64,
                col2 as i64,
            );
            distance_sum += distance;
        }
    }
    distance_sum
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let mut space = process_input(&input);
    expand_universe(&mut space);
    let distance = compute_distance_between_galaxies(&space, &1);
    return distance.to_string();
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
