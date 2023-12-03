use test_utils::{get_file_content, get_full_path, write_file_content};

struct GenerateCandidates<'a> {
    candidates: Vec<bool>,
    prev_line: &'a str,
    current_line: &'a str,
    next_line: &'a str,
}

impl<'a> GenerateCandidates<'a> {
    fn new(prev_line: &'a str, current_line: &'a str, next_line: &'a str) -> Self {
        let mut instance: GenerateCandidates<'a> = GenerateCandidates {
            candidates: vec![false; current_line.len()],
            prev_line,
            current_line,
            next_line,
        };

        instance.check_candidates();
        instance
    }

    fn check_candidates(&mut self) {
        for eval_line in vec![self.prev_line, self.current_line, self.next_line] {
            for (i, c) in eval_line.chars().enumerate() {
                if c.is_numeric() || c == '.' || c == '\n' {
                    continue;
                }
                if i > 0 {
                    self.candidates[i - 1] = true;
                }
                self.candidates[i] = true;
                if i < self.candidates.len() - 1 {
                    self.candidates[i + 1] = true;
                }
            }
        }
    }

    fn get_candidates(&self) -> Vec<bool> {
        self.candidates.clone()
    }
}

pub struct NumberFromCandidates<'a> {
    candidates: Vec<bool>,
    eval_line: &'a str,
}

impl NumberFromCandidates<'_> {
    pub fn new<'a>(candidates: Vec<bool>, eval_line: &'a str) -> NumberFromCandidates<'a> {
        return NumberFromCandidates {
            candidates,
            eval_line,
        };
    }

    fn get_number_at(&mut self, index: usize) -> u32 {
        let mut number_name: String = String::new();

        // Search left
        for i in (0..index).rev() {
            if !self.eval_line.chars().nth(i).unwrap().is_numeric() {
                break;
            }
            number_name = self.eval_line.chars().nth(i).unwrap().to_string() + &number_name;
            self.candidates[i] = false;
        }
        // Search right
        for i in index..self.eval_line.len() {
            if !self.eval_line.chars().nth(i).unwrap().is_numeric() {
                break;
            }
            number_name.push(self.eval_line.chars().nth(i).unwrap());
            self.candidates[i] = false;
        }

        number_name = number_name.chars().collect();
        return number_name.parse().unwrap();
    }

    pub fn get_numbers_in_candidates(&mut self) -> Vec<u32> {
        let mut numbers: Vec<u32> = Vec::new();

        for i in 0..self.candidates.len() {
            if self.candidates[i] {
                let c: char = self.eval_line.chars().nth(i).unwrap();
                if c.is_numeric() {
                    let number: u32 = self.get_number_at(i);
                    numbers.push(number);
                }
            }
        }
        return numbers;
    }

    pub fn get_numbers_sum(&mut self) -> u32 {
        let numbers: Vec<u32> = self.get_numbers_in_candidates();
        let mut sum: u32 = 0;
        for number in numbers {
            sum += number;
        }
        return sum;
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
        let candidates: Vec<bool> =
            GenerateCandidates::new(&prev_line, &current_line, &next_line).get_candidates();
        let mut number_from_candidates: NumberFromCandidates =
            NumberFromCandidates::new(candidates, current_line);
        sum += number_from_candidates.get_numbers_sum();
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
