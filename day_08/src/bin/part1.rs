use std::collections::HashMap;
use test_utils::{get_file_content, get_full_path, write_file_content};

pub fn process_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    // Get first line
    let instructions = input.lines().next().unwrap().to_string();

    let mut nodes = HashMap::new();

    // For the rest of lines
    for line in input.lines().skip(1) {
        // AAA = (BBB, CCC)

        // Get first part of line
        let node = line.split(" = ").next().unwrap().to_string();

        // Get second part of line
        let second_part: &str = line.split(" = ").last().unwrap();

        // Remove brackets
        let second_part = second_part.trim_start_matches('(');
        let second_part = second_part.trim_end_matches(')');

        let node_left = second_part.split(", ").next().unwrap().to_string();
        let node_right = second_part.split(", ").last().unwrap().to_string();

        // Insert node_left and node_right
        nodes.insert(node, (node_left, node_right));
    }
    return (instructions, nodes);
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let (instructions, nodes) = process_input(&input);

    let mut steps: i64 = 0;
    let mut instructions_index: usize = 0;
    let instructions_chars: Vec<char> = instructions.chars().collect();

    let mut current_node: String = "AAA".to_string();

    loop {
        if instructions_index == instructions_chars.len() {
            instructions_index = 0;
        }
        let instruction: char = instructions_chars[instructions_index];

        let next_node = match instruction {
            'R' => nodes.get(&current_node).map(|x| &x.1),
            'L' => nodes.get(&current_node).map(|x| &x.0),
            _ => unreachable!("Invalid instruction!"),
        };
        steps += 1;

        current_node = next_node.unwrap().to_string();
        if current_node == "ZZZ" {
            break;
        }

        instructions_index += 1;
    }
    return steps.to_string();
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
