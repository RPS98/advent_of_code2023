use test_utils::{get_file_content, get_full_path, write_file_content};
mod part1;

fn process_seeds(seeds: &Vec<u64>) -> Vec<(u64, u64)> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for index in 0..seeds.len() {
        if index % 2 == 0 {
            let start: u64 = seeds[index];
            let length = seeds[index + 1];
            ranges.push((start, length));
        }
    }
    ranges.sort();
    return ranges;
}

fn get_minimun_location(seeds_ranges: &Vec<(u64, u64)>, maps: &part1::Maps) -> u64 {
    let mut minumun_location: u64 = u64::MAX;

    for seed_range in seeds_ranges {
        let seed_start = seed_range.0;
        let seed_length = seed_range.1;

        for seed in seed_start..(seed_start + seed_length) {
            let minumun_location_aux = part1::get_seed_location(seed, &maps);

            if minumun_location_aux < minumun_location {
                minumun_location = minumun_location_aux;
            }
        }
    }
    return minumun_location;
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let (seeds, maps): (Vec<u64>, part1::Maps) = part1::process_input(&input);

    let processed_seeds: Vec<(u64, u64)> = process_seeds(&seeds);

    let minumun_location: u64 = get_minimun_location(&processed_seeds, &maps);

    return minumun_location.to_string();
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
