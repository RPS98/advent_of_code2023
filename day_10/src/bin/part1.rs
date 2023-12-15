use colored::*;
use day_10::Matrix;
use std::fmt;
use test_utils::{get_file_content, get_full_path, write_file_content};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Start,
    V,
    H,
    NE,
    NW,
    SW,
    SE,
    None,
    Inner,
    Outter,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Start => write!(f, "S"),
            Direction::V => write!(f, "\u{2502}"),
            Direction::H => write!(f, "\u{2500}"),
            Direction::NE => write!(f, "\u{2514}"),
            Direction::NW => write!(f, "\u{2518}"),
            Direction::SW => write!(f, "\u{2510}"),
            Direction::SE => write!(f, "\u{250C}"),
            Direction::None => write!(f, "{}", ".".bright_black()),
            Direction::Inner => write!(f, "{}", "I".green()),
            Direction::Outter => write!(f, "{}", "O".red()),
        }
    }
}

fn char_to_direction(ch: char) -> Direction {
    match ch {
        'S' => Direction::Start,
        '|' => Direction::V,
        '-' => Direction::H,
        'L' => Direction::NE,
        'J' => Direction::NW,
        '7' => Direction::SW,
        'F' => Direction::SE,
        '.' => Direction::None,
        _ => Direction::None,
    }
}

pub fn process_input(input: &str) -> (Matrix<Direction>, (usize, usize)) {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();
    let mut matrix: Matrix<Direction> = Matrix::new(num_rows, num_cols, Direction::None);
    let mut start_coords: (usize, usize) = (0, 0);
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            matrix.set_value(char_to_direction(ch), row, col);
            if *matrix.at(row, col) == Direction::Start {
                start_coords = (row, col);
            }
        }
    }
    return (matrix, start_coords);
}

fn get_possibles_start_movement(
    matrix: &Matrix<Direction>,
    start_coords: &(usize, usize),
) -> (i8, i8, i8, i8) {
    // (N, S, W, E)
    let mut possibles: (i8, i8, i8, i8) = (0, 0, 0, 0);

    // North cell
    if start_coords.0 > 0 {
        possibles.0 = match matrix.at(start_coords.0 - 1, start_coords.1) {
            Direction::V => 1,
            Direction::SW => 1,
            Direction::SE => 1,
            _ => 0,
        };
    }
    // South cell
    if start_coords.0 < matrix.rows - 1 {
        possibles.1 = match matrix.at(start_coords.0 + 1, start_coords.1) {
            Direction::V => 1,
            Direction::NW => 1,
            Direction::NE => 1,
            _ => 0,
        };
    }
    // West cell
    if start_coords.1 > 0 {
        possibles.2 = match matrix.at(start_coords.0, start_coords.1 - 1) {
            Direction::H => 1,
            Direction::NE => 1,
            Direction::SE => 1,
            _ => 0,
        };
    }
    // East cell
    if start_coords.1 < matrix.cols - 1 {
        possibles.3 = match matrix.at(start_coords.0, start_coords.1 + 1) {
            Direction::H => 1,
            Direction::NW => 1,
            Direction::SW => 1,
            _ => 0,
        };
    }

    return possibles;
}

fn get_possibles_movement(direction: &Direction) -> (i8, i8, i8, i8) {
    // (N, S, W, E)
    let posibles: (i8, i8, i8, i8) = match direction {
        Direction::Start => (1, 1, 1, 1),
        Direction::V => (1, 1, 0, 0),
        Direction::H => (0, 0, 1, 1),
        Direction::NE => (1, 0, 0, 1),
        Direction::NW => (1, 0, 1, 0),
        Direction::SW => (0, 1, 1, 0),
        Direction::SE => (0, 1, 0, 1),
        _ => (0, 0, 0, 0),
    };
    return posibles;
}

fn movement_to_direction(direction: &(i8, i8, i8, i8)) -> Direction {
    // (N, S, W, E)
    let mut dir: Direction = Direction::None;
    if direction.0 == 1 && direction.1 == 1 {
        dir = Direction::V;
    } else if direction.2 == 1 && direction.3 == 1 {
        dir = Direction::H;
    } else if direction.0 == 1 && direction.3 == 1 {
        dir = Direction::NE;
    } else if direction.0 == 1 && direction.2 == 1 {
        dir = Direction::NW;
    } else if direction.1 == 1 && direction.2 == 1 {
        dir = Direction::SW;
    } else if direction.1 == 1 && direction.3 == 1 {
        dir = Direction::SE;
    }
    return dir;
}

fn movement_to_coord_vec(mov: &(i8, i8, i8, i8), coord: &(usize, usize)) -> Vec<(usize, usize)> {
    // (N, S, W, E)
    let mut coords: Vec<(usize, usize)> = Vec::new();
    if mov.0 == 1 {
        coords.push((coord.0 - 1, coord.1));
    }
    if mov.1 == 1 {
        coords.push((coord.0 + 1, coord.1));
    }
    if mov.2 == 1 {
        coords.push((coord.0, coord.1 - 1));
    }
    if mov.3 == 1 {
        coords.push((coord.0, coord.1 + 1));
    }
    return coords;
}

pub fn find_path(
    matrix: &Matrix<Direction>,
    start_coords: (usize, usize),
) -> (Matrix<Direction>, Matrix<i8>) {
    let mut path: Matrix<Direction> = Matrix::new(matrix.rows, matrix.cols, Direction::None);
    let mut visited_cells: Matrix<i8> = Matrix::new(matrix.rows, matrix.cols, 0);

    visited_cells.set_value(1, start_coords.0, start_coords.1);
    path.set_value(Direction::Start, start_coords.0, start_coords.1);

    let starts_mov = get_possibles_start_movement(&matrix, &start_coords);
    let mut eval_coords: Vec<(usize, usize)> = movement_to_coord_vec(&starts_mov, &start_coords);

    while !eval_coords.is_empty() {
        let mut eval_coords_aux: Vec<(usize, usize)> = Vec::new();
        for coords in eval_coords.iter() {
            path.set_value(*matrix.at(coords.0, coords.1), coords.0, coords.1);

            let start_possibles = get_possibles_movement(matrix.at(coords.0, coords.1));

            if start_possibles.0 == 1 && coords.0 > 0 {
                if *visited_cells.at(coords.0 - 1, coords.1) == 0 {
                    eval_coords_aux.push((coords.0 - 1, coords.1));
                }
            }
            if start_possibles.1 == 1 && coords.0 < matrix.rows - 1 {
                if *visited_cells.at(coords.0 + 1, coords.1) == 0 {
                    eval_coords_aux.push((coords.0 + 1, coords.1));
                }
            }
            if start_possibles.2 == 1 && coords.1 > 0 {
                if *visited_cells.at(coords.0, coords.1 - 1) == 0 {
                    eval_coords_aux.push((coords.0, coords.1 - 1));
                }
            }
            if start_possibles.3 == 1 && coords.1 < matrix.cols - 1 {
                if *visited_cells.at(coords.0, coords.1 + 1) == 0 {
                    eval_coords_aux.push((coords.0, coords.1 + 1));
                }
            }
            visited_cells.set_value(1, coords.0, coords.1)
        }
        eval_coords = eval_coords_aux;
    }
    path.set_value(
        movement_to_direction(&starts_mov),
        start_coords.0,
        start_coords.1,
    );
    return (path, visited_cells);
}

pub fn get_path_length(matrix: &Matrix<i8>) -> i64 {
    let mut path_length: i64 = 0;
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            if *matrix.at(i, j) == 1 {
                path_length += 1;
            }
        }
    }
    return path_length;
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let (matrix, start_coords) = process_input(&input);
    let (path, visited_cells) = find_path(&matrix, start_coords);
    let farest_length = get_path_length(&visited_cells) / 2;
    println!("Path:\n{}", path);
    return farest_length.to_string();
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
