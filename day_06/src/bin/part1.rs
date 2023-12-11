use test_utils::{get_file_content, get_full_path, write_file_content};

pub struct Race {
    pub time: i64,
    pub distance: i64,
}

impl Race {
    fn wrap_time(&self, press_time_f: f64) -> i64 {
        let press_time: i64 = press_time_f as i64;

        if press_time < 0 {
            return 0;
        } else if press_time > self.time {
            return self.time;
        }
        return press_time;
    }

    // Race distance = (race_time - press_time) * press_time
    // press_time² - race_time * press_time + race_distance = 0
    // press_time = -b ± √(b² - 4ac) / 2a
    // press_time = -race_time ± √(race_time² - 4·1·race_distance) / 2
    fn solve_press_time(&self) -> (i64, i64) {
        let a: f64 = 1.0;
        let b: f64 = -self.time as f64;
        let c: f64 = self.distance as f64;
        let delta: f64 = b * b - 4.0 * a * c;
        if delta < 0.0 {
            panic!("Delta is negative!");
        }

        let mut press_time_pos: f64 = (-b + delta.sqrt()) / 2.0;
        let press_time_neg: f64 = (-b - delta.sqrt()) / 2.0 + 1.0;

        // If the root is exact, it doesn't win the race
        if press_time_pos.fract() == 0.0 {
            press_time_pos -= 1.0;
        }
        let press_time_pos: i64 = self.wrap_time(press_time_pos);
        let press_time_neg: i64 = self.wrap_time(press_time_neg);

        return (press_time_pos, press_time_neg);
    }

    pub fn get_num_press_time_options(&self) -> i64 {
        let (press_time_pos, press_time_neg) = self.solve_press_time();
        let num_press_time_options: i64 = press_time_pos - press_time_neg + 1;
        return num_press_time_options.abs() as i64;
    }
}

pub fn process_input(input: &str) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();
    // Input format:
    // Time:      7  15   30
    // Distance:  9  40  200

    let time_line: &str = input.lines().nth(0).unwrap();
    let distance_line: &str = input.lines().nth(1).unwrap();

    let time_line: String = time_line.replace("Time: ", "");
    let distance_line: String = distance_line.replace("Distance: ", "");

    let time_values: Vec<&str> = time_line.split_whitespace().collect();
    let distance_values: Vec<&str> = distance_line.split_whitespace().collect();

    if time_values.len() != distance_values.len() {
        panic!("Time and distance have different length!");
    }

    for (time, distance) in time_values.iter().zip(distance_values.iter()) {
        let race: Race = Race {
            time: time.parse::<i64>().unwrap(),
            distance: distance.parse::<i64>().unwrap(),
        };
        races.push(race);
    }

    return races;
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);

    let races: Vec<Race> = process_input(&input);
    let mut total_options: i64 = 1;
    for race in races {
        let num_press_time_options = race.get_num_press_time_options();
        total_options *= num_press_time_options;
    }

    return total_options.to_string();
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
