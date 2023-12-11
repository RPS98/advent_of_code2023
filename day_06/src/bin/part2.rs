use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;

pub fn process_input(input: &str) -> part1::Race {
    let time_line: &str = input.lines().nth(0).unwrap();
    let distance_line: &str = input.lines().nth(1).unwrap();

    let time_line: String = time_line.replace("Time: ", "");
    let distance_line: String = distance_line.replace("Distance: ", "");

    let time_line: String = time_line.replace(" ", "");
    let distance_line: String = distance_line.replace(" ", "");

    let race_time: i64 = time_line.parse::<i64>().unwrap();
    let race_distance: i64 = distance_line.parse::<i64>().unwrap();

    let race: part1::Race = part1::Race {
        time: race_time,
        distance: race_distance,
    };

    return race;
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);

    let race: part1::Race = process_input(&input);
    let num_press_time_options = race.get_num_press_time_options();

    return num_press_time_options.to_string();
}

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
