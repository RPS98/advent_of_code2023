use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;
use part1::{find_path, process_input, Direction};

fn get_countour_cross(contour: &Vec<Direction>) -> i8 {
    // Count number of Direction::V, Direction::NE, Direction::NW
    let mut num_vectical: i8 = 0;
    let mut num_north_east: i8 = 0;
    let mut num_north_west: i8 = 0;
    let mut num_south_east: i8 = 0;
    let mut num_south_west: i8 = 0;
    for direction in contour {
        match direction {
            Direction::V => num_vectical += 1,
            Direction::NE => num_north_east += 1,
            Direction::NW => num_north_west += 1,
            Direction::SE => num_south_east += 1,
            Direction::SW => num_south_west += 1,
            _ => (),
        }
    }

    let min_num_0: i8 = std::cmp::min(num_north_east, num_south_west);
    let min_num_1: i8 = std::cmp::min(num_north_west, num_south_east);

    let countour_cross: i8 = num_vectical + min_num_0 + min_num_1;

    return countour_cross;
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let (matrix, start_coords) = process_input(&input);
    let (mut path, visited_cells) = find_path(&matrix, start_coords);

    let mut inner_points: Vec<(usize, usize)> = Vec::new();
    for row in 1..visited_cells.rows - 1 {
        for col in 1..visited_cells.cols - 1 {
            if *visited_cells.at(row, col) == 0 {
                // let visited_cells_left: Vec<i8> = visited_cells.get_row_vector(j, 0, i);
                path.set_value(Direction::Outter, row, col);

                let left_part = path.get_row_vector(row, 0, col);
                if left_part.len() > 1 {
                    let contour_cross = get_countour_cross(&left_part);
                    if contour_cross % 2 == 1 {
                        path.set_value(Direction::Inner, row, col);
                        inner_points.push((row, col));
                    }
                }
            }
        }
    }

    println!("Path:\n{}", path);
    return inner_points.len().to_string();
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
