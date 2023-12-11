use test_utils::{get_file_content, get_full_path, write_file_content};

#[derive(Debug)]
pub struct MapRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

#[derive(Debug)]
pub struct MapRanges {
    map_ranges: Vec<MapRange>,
}

impl MapRanges {
    pub fn get_map_corresponding_value(&self, value: u64) -> u64 {
        for map_range in &self.map_ranges {
            if value >= map_range.source_start && value < map_range.source_start + map_range.length
            {
                return value - map_range.source_start + map_range.destination_start;
            }
        }
        return value;
    }
}

#[derive(Debug)]
pub struct Maps {
    seed_to_soil: MapRanges,
    soil_to_fertilizer: MapRanges,
    fertilizer_to_water: MapRanges,
    water_to_light: MapRanges,
    light_to_temperature: MapRanges,
    temperature_to_humidity: MapRanges,
    humidity_to_location: MapRanges,
}

pub fn process_input(input: &str) -> (Vec<u64>, Maps) {
    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Maps = Maps {
        seed_to_soil: MapRanges { map_ranges: vec![] },
        soil_to_fertilizer: MapRanges { map_ranges: vec![] },
        fertilizer_to_water: MapRanges { map_ranges: vec![] },
        water_to_light: MapRanges { map_ranges: vec![] },
        light_to_temperature: MapRanges { map_ranges: vec![] },
        temperature_to_humidity: MapRanges { map_ranges: vec![] },
        humidity_to_location: MapRanges { map_ranges: vec![] },
    };

    let mut current_map_ranges: Option<&mut MapRanges> = None;
    let mut lines: std::str::Lines<'_> = input.lines();

    for line in &mut lines {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("seeds: ") {
            seeds = line
                .split(" ")
                .skip(1)
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            continue;
        }

        if line.ends_with("map:") {
            current_map_ranges = match line {
                "seed-to-soil map:" => Some(&mut maps.seed_to_soil),
                "soil-to-fertilizer map:" => Some(&mut maps.soil_to_fertilizer),
                "fertilizer-to-water map:" => Some(&mut maps.fertilizer_to_water),
                "water-to-light map:" => Some(&mut maps.water_to_light),
                "light-to-temperature map:" => Some(&mut maps.light_to_temperature),
                "temperature-to-humidity map:" => Some(&mut maps.temperature_to_humidity),
                "humidity-to-location map:" => Some(&mut maps.humidity_to_location),
                _ => continue,
            };
            continue;
        }

        if current_map_ranges.is_some() {
            let mut map_range: MapRange = MapRange {
                destination_start: 0,
                source_start: 0,
                length: 0,
            };

            let line_parts: Vec<u64> = line.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();

            map_range.destination_start = line_parts[0];
            map_range.source_start = line_parts[1];
            map_range.length = line_parts[2];

            current_map_ranges
                .as_mut()
                .unwrap()
                .map_ranges
                .push(map_range);
        }
    }

    (seeds, maps)
}

pub fn get_seed_location(seed: u64, maps: &Maps) -> u64 {
    let soil: u64 = maps.seed_to_soil.get_map_corresponding_value(seed);
    let fertilizer: u64 = maps.soil_to_fertilizer.get_map_corresponding_value(soil);
    let water: u64 = maps
        .fertilizer_to_water
        .get_map_corresponding_value(fertilizer);
    let light: u64 = maps.water_to_light.get_map_corresponding_value(water);
    let temperature: u64 = maps.light_to_temperature.get_map_corresponding_value(light);
    let humidity: u64 = maps
        .temperature_to_humidity
        .get_map_corresponding_value(temperature);
    let location: u64 = maps
        .humidity_to_location
        .get_map_corresponding_value(humidity);
    return location;
}

fn get_minimun_location(seeds: &Vec<u64>, maps: &Maps) -> u64 {
    let mut minumun_location: u64 = u64::MAX;
    for seed in seeds {
        let location: u64 = get_seed_location(*seed, &maps);
        if location < minumun_location {
            minumun_location = location;
        }
    }
    return minumun_location;
}

fn advent_of_code(input_text: &str) -> String {
    let input: String = String::from(input_text);
    let (seeds, maps): (Vec<u64>, Maps) = process_input(&input);

    let minumun_location: u64 = get_minimun_location(&seeds, &maps);

    return minumun_location.to_string();
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
