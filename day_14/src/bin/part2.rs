use std::collections::HashMap;

use lib_utils::Matrix;
use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;
use part1::{compute_load, move_platform, process_input, Direction};

fn process_cycle(platform: &mut Matrix<u8>) {
    let mut direction = Direction::North;
    move_platform(platform, &direction);
    direction = Direction::West;
    move_platform(platform, &direction);
    direction = Direction::South;
    move_platform(platform, &direction);
    direction = Direction::East;
    move_platform(platform, &direction);
}

fn cycles_move(platform: &mut Matrix<u8>, cycles: u64) {
    let mut cache: HashMap<Vec<u8>, u64> = HashMap::new();
    let mut cycle = 0;

    while cycle < cycles {
        let key = platform.data.clone();
        if cache.contains_key(&key) {
            let cycle_len = cycle - cache.get(&key).unwrap();
            let remaining_cycles = cycles - cycle;
            let cycles_to_skip = remaining_cycles / cycle_len;
            cycle += cycles_to_skip * cycle_len;
        } else {
            cache.insert(key, cycle);
        }
        process_cycle(platform);
        cycle += 1;
    }
}

fn advent_of_code(input_text: &str) -> String {
    let mut platform: Matrix<u8> = process_input(input_text);
    cycles_move(&mut platform, 1000000000);
    let load = compute_load(&platform);
    return load.to_string();
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
