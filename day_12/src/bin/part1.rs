use std::collections::HashMap;
use test_utils::{get_file_content, get_full_path, write_file_content};

pub struct SpringsMap {
    pub cfg: String,
    pub groups: Vec<u8>,
}

impl SpringsMap {
    fn extend_line(input_line: &str, factor: u8) -> String {
        let mut extended_line = String::new();

        let mut parts = input_line.split(' ');
        if let (Some(cfg), Some(groups)) = (parts.next(), parts.last()) {
            let mut extended_cfg = String::from(cfg);
            let mut extended_groups = String::from(groups);

            for _ in 1..factor {
                extended_cfg.push('?');
                extended_cfg.push_str(cfg);

                extended_groups.push(',');
                extended_groups.push_str(groups);
            }

            extended_line.push_str(&extended_cfg);
            extended_line.push(' ');
            extended_line.push_str(&extended_groups);
        }

        extended_line
    }

    fn new(input_line: &str, factor: u8) -> SpringsMap {
        let input_line_extended = SpringsMap::extend_line(input_line, factor);
        let mut parts = input_line_extended.split(' ');
        if let (Some(cfg), Some(groups)) = (parts.next(), parts.last()) {
            let mut groups_vec: Vec<u8> = Vec::new();
            for group in groups.split(',') {
                groups_vec.push(group.parse::<u8>().unwrap());
            }
            let cfg = cfg.to_string();
            SpringsMap {
                cfg,
                groups: groups_vec,
            }
        } else {
            panic!("Invalid input line!");
        }
    }
}

pub fn process_input(input: &str, factor: u8) -> Vec<SpringsMap> {
    let mut springs_map: Vec<SpringsMap> = Vec::new();

    for line in input.lines() {
        springs_map.push(SpringsMap::new(line, factor));
    }
    springs_map
}

pub fn count_valid_arrangements(
    cfg: &str,
    groups: &Vec<u8>,
    cache: &mut HashMap<(String, Vec<u8>), u64>,
) -> u64 {
    let mut cont = 0;

    if cfg.is_empty() {
        if groups.is_empty() {
            // Is valid
            cont += 1;
        }
        return cont;
    }

    if groups.is_empty() {
        if !cfg.contains("#") {
            // Is valid
            cont += 1;
        }
        return cont;
    }

    // Check cache
    let key = (cfg.to_string(), groups.clone());
    if let Some(value) = cache.get(&key) {
        return *value;
    }

    let first_char = cfg.chars().next().unwrap();

    // Evaluating '?' as '.'
    if first_char == '.' || first_char == '?' {
        let rest_cfg = &cfg[1..];
        cont += count_valid_arrangements(rest_cfg, groups, cache);
    }

    // Evaluating '?' as '#'
    if first_char == '#' || first_char == '?' {
        // Check if can be valid configuration
        // If group[0] is greater than cfg.len(), then is not valid
        // If group_cfg contais '.', then is not valid
        // If cfg[0..group[0] + 1] i '#', then is not valid

        let first_group = groups[0];
        if cfg.len() as u8 >= first_group {
            let group_cfg = &cfg[..first_group as usize];
            if !group_cfg.contains('.') {
                let rest_cfg = &cfg[first_group as usize..];
                let mut rest_groups = groups.clone();
                rest_groups.remove(0);

                if rest_cfg.is_empty() {
                    cont += count_valid_arrangements(rest_cfg, &rest_groups, cache);
                } else {
                    let next_cfg_char = rest_cfg.chars().next().unwrap();
                    if next_cfg_char != '#' {
                        let rest_cfg = &rest_cfg[1..];
                        cont += count_valid_arrangements(rest_cfg, &rest_groups, cache);
                    }
                }
            }
        }
    }

    // Add to cache
    cache.insert(key, cont);

    return cont;
}

fn advent_of_code(input_text: &str) -> String {
    let mut valid_arrangements_sum = 0;

    let input: String = String::from(input_text);
    let springs_map = process_input(&input, 1);

    for sm in springs_map {
        let mut cache = &mut HashMap::new();
        let groups = sm.groups.clone();
        let valid_arrangements = count_valid_arrangements(&sm.cfg, &groups, &mut cache);
        valid_arrangements_sum += valid_arrangements;
    }
    return valid_arrangements_sum.to_string();
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
