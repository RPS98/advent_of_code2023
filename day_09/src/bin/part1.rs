use test_utils::{get_file_content, get_full_path, write_file_content};

pub fn find_sequence(data: &Vec<i64>) -> i64 {
    let mut differences: Vec<i64> = Vec::new();
    let mut prev_number = data[0];

    if data.len() == 1 {
        return data[0];
    }

    if data.iter().all(|&x| x == 0) {
        return 0;
    }

    for i in 1..data.len() {
        let diff = data[i] - prev_number;
        prev_number = data[i];
        differences.push(diff);
    }

    let data_last = data.last().unwrap();
    return data_last + find_sequence(&differences);
}

pub fn sum_sequence(data: Vec<Vec<i64>>) -> i64 {
    let mut sequence: i64 = 0;
    for line in data {
        let sequence_i = find_sequence(&line);
        sequence += sequence_i;
    }
    return sequence;
}

pub fn process_input(input: &str) -> Vec<Vec<i64>> {
    let mut output: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let mut line_vec: Vec<i64> = Vec::new();
        for num in line.split(" ") {
            line_vec.push(num.parse::<i64>().unwrap());
        }
        output.push(line_vec);
    }
    return output;
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let data = process_input(&input);
    return sum_sequence(data).to_string();
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
