use test_utils::{get_file_content, get_full_path, write_file_content};

pub fn get_max_value_per_color(line_text: &str) -> (u32, u32, u32, u32) {
    // Find index of first :
    let index = line_text.find(':').unwrap_or(0);

    let game_id: u32 = line_text[5..index].parse().unwrap_or(0);
    let line_colors: &str = &line_text[index + 2..];
    let line_colors: String = line_colors.replace(",", ";");
    let colors: Vec<&str> = line_colors.split(';').collect();

    let mut max_blue: u32 = 0;
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    for color_str in colors {
        let color: Vec<&str> = color_str.trim().split_whitespace().collect();
        let color_count: u32 = color[0].parse().unwrap_or(0);
        let color_name: &str = color[1];
        match color_name {
            "blue" => {
                if color_count > max_blue {
                    max_blue = color_count;
                }
            }
            "red" => {
                if color_count > max_red {
                    max_red = color_count;
                }
            }
            "green" => {
                if color_count > max_green {
                    max_green = color_count;
                }
            }
            _ => {
                println!("Invalid color: {}", color_name);
                break;
            }
        };
    }
    return (game_id, max_blue, max_red, max_green);
}

fn advent_of_code(input_text: &str) -> String {
    let mut games_available: Vec<u32> = Vec::new();

    let max_blue_expected: u32 = 14;
    let max_red_expected: u32 = 12;
    let max_green_expected: u32 = 13;

    for line in input_text.lines() {
        let (game_id, max_blue, max_red, max_green) = get_max_value_per_color(line);
        if max_blue <= max_blue_expected
            && max_red <= max_red_expected
            && max_green <= max_green_expected
        {
            games_available.push(game_id);
        }
    }

    // Sum all id in games_available
    let mut sum: u32 = 0;
    for game_id in games_available {
        sum += game_id;
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
