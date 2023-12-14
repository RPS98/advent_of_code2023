use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;
use num_integer;

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let (instructions, nodes) = part1::process_input(&input);

    let starts_node: Vec<String> = nodes
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|x| x.to_string())
        .collect();

    let ends_node: Vec<String> = nodes
        .keys()
        .filter(|x| x.ends_with('Z'))
        .map(|x| x.to_string())
        .collect();

    let mut steps: i128 = 0;
    let mut instructions_index: usize = 0;
    let instructions_chars = instructions.chars().collect::<Vec<char>>();
    let mut current_nodes: Vec<String> = starts_node.clone();
    let mut last_step: Vec<i128> = vec![0; current_nodes.len()];
    let mut cycles_steps: Vec<i128> = vec![0; current_nodes.len()];

    'outer: loop {
        if instructions_index == instructions_chars.len() {
            instructions_index = 0;
        }

        steps += 1;

        for i in 0..current_nodes.len() {
            let current_node = &current_nodes[i];

            let next_node = match instructions_chars[instructions_index] {
                'R' => nodes.get(current_node).map(|x| &x.1),
                'L' => nodes.get(current_node).map(|x| &x.0),
                _ => panic!("Invalid instruction!"),
            };

            // Check if next node is in ends_node
            if next_node.is_none() {
                !panic!("Invalid instruction!");
            }
            let next_node = next_node.unwrap().to_string();

            if cycles_steps[i] == 0 && ends_node.contains(&next_node) {
                cycles_steps[i] = steps - last_step[i];
                last_step[i] = steps;

                // Check if all cycles_steps are different from 0
                if cycles_steps.iter().all(|&x| x != 0) {
                    break 'outer;
                }
            }
            current_nodes[i] = next_node;
        }
        instructions_index += 1;
    }

    let mut lcm_steps: usize = cycles_steps[0] as usize;
    for i in 1..cycles_steps.len() {
        lcm_steps = num_integer::lcm(lcm_steps, cycles_steps[i] as usize);
    }
    return lcm_steps.to_string();
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
