use std::fs;

pub fn test_advent_of_code(
    input_test: &str,
    output_test: &str,
    func: impl Fn(&str) -> String,
) -> bool {
    // Read content from the input_test file
    let input_content: String = fs::read_to_string(input_test).expect("Unable to read input file.");
    if input_content.is_empty() {
        println!("The input file is empty.");
        return false;
    }

    // Get the result of applying the function to the read content
    let result: String = func(&input_content);

    // Read content from the output_test file
    let output_content: String =
        fs::read_to_string(output_test).expect("Unable to read output file.");

    // Compare results
    println!("Result obtained: {}", result);
    println!("Result expected: {}", output_content);
    result == output_content
}

pub fn get_full_path(file_name: &str) -> String {
    if let Ok(full_path) = std::fs::canonicalize(file_name) {
        if let Some(path_str) = full_path.to_str() {
            return String::from(path_str);
        } else {
            println!("Unable to convert the path to a string.");
        }
    } else {
        println!("Unable to get full path for the file: {}", file_name);
    }

    String::new() // Return an empty string in case of failure
}

pub fn get_file_content(file_name: &str) -> String {
    let full_path: String = get_full_path(file_name);

    if full_path.is_empty() {
        return String::new();
    }

    if let Ok(content) = fs::read_to_string(full_path) {
        return content;
    }

    String::new() // Return an empty string in case of failure
}

pub fn write_file_content(file_name: &str, content: &str) -> bool {
    if content.is_empty() {
        panic!("Input is empty!");
    }

    let full_path: String = get_full_path(file_name);

    if full_path.is_empty() {
        return false;
    }

    if let Ok(_) = fs::write(full_path, content) {
        return true;
    }

    false // Return false in case of failure
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test() {
        let input_test_path: &Path = Path::new("src/test/input_test_file.txt");
        let output_test_path: &Path = Path::new("src/test/output_test_file.txt");

        let identity_function = |s: &str| s.to_owned();

        let result: bool = test_advent_of_code(
            input_test_path.to_str().unwrap(),
            output_test_path.to_str().unwrap(),
            identity_function,
        );

        assert!(result, "The files are not equal.");
    }
}
