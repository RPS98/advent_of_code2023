use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;
use part1::{
    compute_distance_between_galaxies, compute_manhattan_distance_between, expand_universe,
    get_galaxies_coordinates, process_input,
};

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let mut space = process_input(&input);
    expand_universe(&mut space);
    // Substract 1 beacuse i am adding the row/col, not replacing it
    let factor = 100 - 1;
    // let factor = 1000000 - 1;
    let distance = compute_distance_between_galaxies(&space, &factor);
    return distance.to_string();
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
