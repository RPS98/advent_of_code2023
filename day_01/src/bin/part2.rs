use std::collections::HashMap;
use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;

struct NumberNames {
    number_map: HashMap<String, char>,
    starting_with_map: HashMap<char, Vec<String>>,
}

impl NumberNames {
    fn new() -> Self {
        let mut number_map: HashMap<String, char> = HashMap::new();
        let mut starting_with_map: HashMap<char, Vec<String>> = HashMap::new();

        let names: Vec<String> = vec![
            String::from("zero"),
            String::from("one"),
            String::from("two"),
            String::from("three"),
            String::from("four"),
            String::from("five"),
            String::from("six"),
            String::from("seven"),
            String::from("eight"),
            String::from("nine"),
        ];

        for (i, name) in names.iter().enumerate() {
            let i_char: Option<char> = char::from_digit(i as u32, 10);
            number_map.insert(name.clone(), i_char.unwrap());
            starting_with_map
                .entry(name.chars().next().unwrap())
                .or_insert_with(Vec::new)
                .push(name.clone());
        }

        NumberNames {
            number_map,
            starting_with_map,
        }
    }

    fn names_starting_with_and_length(&self, character: char, len: usize) -> Option<Vec<&String>> {
        if let Some(starting_with) = self.starting_with_map.get(&character) {
            let names: Vec<&String> = starting_with
                .iter()
                .filter(|&name| name.len() <= len)
                .collect();
            return Some(names);
        }
        None
    }

    fn get_number_at_beginning(&self, string_data: &str) -> Option<String> {
        let string_len: usize = string_data.len();
        let first_char: char = string_data.chars().next().unwrap();

        if let Some(candidates) = self.names_starting_with_and_length(first_char, string_len) {
            for candidate in candidates {
                if string_data.starts_with(candidate) {
                    return Some(candidate.clone());
                }
            }
        }
        None
    }
}

fn replace_digits_string_to_int(line: &mut String, number_names: &NumberNames) {
    let mut line_formatted: String = String::from("");
    for mut i in 0..line.len() {
        let char_i: char = line.chars().nth(i).unwrap();
        // Check if char i is numeric
        if char_i.is_numeric() {
            line_formatted.push(char_i);
            continue;
        }
        // Check if char i is the beginning of a number name
        let numer_found: Option<String> = number_names.get_number_at_beginning(&line[i..]);
        if numer_found.is_some() {
            let number_name: String = numer_found.unwrap();
            let number_name_len = number_name.len();
            line_formatted.push(number_names.number_map[&number_name]);
            i += number_name_len - 1;
        }
    }
    *line = line_formatted;
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let mut sum: u32 = 0;
    let number_names: NumberNames = NumberNames::new();

    for line_str in input.lines() {
        let mut line: String = line_str.to_string();
        replace_digits_string_to_int(&mut line, &number_names);
        sum += part1::sum_first_and_last_digits(&line);
    }

    return sum.to_string();
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
