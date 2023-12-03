use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;

struct Gear<'a> {
    candidates: Vec<bool>,
    line_prev: &'a str,
    line_current: &'a str,
    line_next: &'a str,
}

impl<'a> Gear<'a> {
    fn new(line_prev: &'a str, line_current: &'a str, line_next: &'a str) -> Gear<'a> {
        Gear {
            candidates: vec![false; line_current.len()],
            line_prev: line_prev,
            line_current: line_current,
            line_next: line_next,
        }
    }

    fn find_numbers(&mut self) -> Vec<u32> {
        let mut numbers: Vec<u32> = Vec::new();
        for eval_line in vec![self.line_prev, self.line_current, self.line_next] {
            let mut numbers_from_candidates: part1::NumberFromCandidates =
                part1::NumberFromCandidates::new(self.candidates.clone(), eval_line);
            numbers.append(&mut numbers_from_candidates.get_numbers_in_candidates());
        }
        return numbers;
    }

    fn evaluate_gear(&mut self, index: usize) -> u32 {
        if index > 0 {
            self.candidates[index - 1] = true;
        }
        self.candidates[index] = true;
        if index < self.candidates.len() - 1 {
            self.candidates[index + 1] = true;
        }

        let numbers: Vec<u32> = self.find_numbers();
        let mut gear_ratio: u32 = 0;
        if numbers.len() == 2 {
            gear_ratio = numbers[0] * numbers[1];
        }

        self.candidates = vec![false; self.line_current.len()];
        return gear_ratio;
    }

    fn get_gear_ratios(&mut self) -> u32 {
        let mut gear_ratios: Vec<u32> = Vec::new();
        for (i, c) in self.line_current.chars().enumerate() {
            if c != '*' {
                continue;
            }

            let gear_ratio: u32 = self.evaluate_gear(i);
            gear_ratios.push(gear_ratio);
        }
        return gear_ratios.iter().sum();
    }
}

fn init_line(line_len: usize, line_char: char) -> String {
    let mut line: String = String::new();
    for _ in 0..line_len {
        line.push(line_char);
    }
    return line;
}

fn advent_of_code(input_text: &str) -> String {
    let mut current_line: &str = "";
    let mut prev_line: &str = "";
    let mut next_line: &str = "";

    let aux_line: String = init_line(input_text.lines().nth(0).unwrap().len(), '.');

    let mut sum: u32 = 0;
    for i in 0..input_text.lines().count() {
        current_line = input_text.lines().nth(i).unwrap();
        if i == 0 {
            prev_line = &aux_line;
        } else {
            prev_line = input_text.lines().nth(i - 1).unwrap();
        }

        if i == input_text.lines().count() - 1 {
            next_line = &aux_line;
        } else {
            next_line = input_text.lines().nth(i + 1).unwrap();
        }
        let mut gear: Gear = Gear::new(prev_line, current_line, next_line);
        sum += gear.get_gear_ratios();
    }
    return sum.to_string();
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
