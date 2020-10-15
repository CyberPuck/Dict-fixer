/// Simple application that reads in a dictionary and removes words with a "'".
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // If the number of arguments don't match, throw an error
    if args.len() != 3 || args.contains(&"-h".to_string()) {
        print_help();
        return;
    }
    let input_file = args.get(1).unwrap();
    let output_file = args.get(2).unwrap();

    let mut dictionary: Vec<String> = Vec::new();

    println!(
        "Input File:\t{:?}\nOutput File:\t{:?}",
        input_file, output_file
    );
    // read in the file
    let result = read_file(&input_file, &mut dictionary);
    if result.is_err() {
        println!("Error, failed to read in file");
        return;
    }

    // modify dictionary
    remove_invalid_words(&mut dictionary);

    // write the fixed dictionary to file
    write_file(output_file, &dictionary);
}

/// Read in a file, given a full file path
/// #Params
/// file_path: &String, representing the path to the input file.
/// words: &mut Vec<String>, dictionary of words to have file data written to.
/// # Returns
/// Result<bool, String>, result of the operation, bool is always true; result is an error string.
fn read_file(file_path: &String, words: &mut Vec<String>) -> Result<bool, String> {
    let file_string_result = fs::read_to_string(file_path);
    if file_string_result.is_err() {
        return Err("Failed to read in file".to_string());
    }
    // parse the string into a vector new line delimeted
    for word in file_string_result.unwrap().split("\n") {
        words.push(word.to_string())
    }
    Ok(true)
}

/// Remove words from the dictionary that contain '.
/// # Params
/// words: &mut Vec<String>, dictionary that will have "'" removed.
/// # Returns
/// None
fn remove_invalid_words(words: &mut Vec<String>) {
    words.retain(|x| !x.contains("'"));
}

/// Writes the valid dictionary words to a file.
/// Note:  The file will be *overwritten*
/// # Params
/// words:  &Vec<String>, dictionary data to be written to file.
/// output_path: &String, path to the output file.
fn write_file(output_path: &String, words: &Vec<String>) {
    let mut full_string: String = String::new();
    for word in words {
        full_string = full_string + word + "\n";
    }
    // NOTE: Remove the last "\n"
    full_string.truncate(full_string.len() - 1);
    if fs::write(output_path, full_string).is_err() {
        println!("Did not write to {}", output_path);
    }
}

/// Prints the help options for the CLI program.
fn print_help() {
    println!("Dictionary Fixer");
    println!("Summary:  Given an input file, strip out all words that contain non-alphebetic characters.");
    println!("\ndict-fixer <input file> <output file> [-h]");
    println!("\tinput file:  New line delimited dictionary to be read in.");
    println!("\toutput file:  File to be created/overwritten with data from input file.");
    println!("\t-h:  Prints this help function.");
}

/// # Unit Testing
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Globals
    const UNIT_TEST_FILE: &str = "unit-test-file.txt";
    const UNIT_TEST_OUTPUT_FILE: &str = "unit-test-output-file.txt";

    /// Support testing function for creating files for unit testing.
    /// # Example
    /// ```
    /// #[test]
    /// fn test() {
    ///     setup_files();
    ///     // ...
    ///     teardown_files();
    /// }
    /// ```
    fn setup_files() {
        let result = fs::write(UNIT_TEST_FILE, "Test\nTest's\nTester\nTested\nTesting");
        if result.is_err() {
            println!("Warning, failed to creat test file: {}", UNIT_TEST_FILE);
        }
    }

    /// Supporting testing function for removing files.
    /// # Example
    /// ```
    /// #[test]
    /// fn test() {
    ///     setup_files();
    ///     // ...
    ///     teardown_files();
    /// }
    /// ```
    fn teardown_files() {
        let result = fs::remove_file(UNIT_TEST_FILE);
        if result.is_err() {
            println!("Warning, failed to remove test file: {}", UNIT_TEST_FILE);
        }
        let result = fs::remove_file(UNIT_TEST_OUTPUT_FILE);
        if result.is_err() {
            println!("Warning, failed to remove test file: {}", UNIT_TEST_FILE);
        }
    }

    #[test]
    fn test_read_file() {
        setup_files();
        let mut words: Vec<String> = Vec::new();
        let result = read_file(&UNIT_TEST_FILE.to_string(), &mut words);
        assert!(!result.is_err());
        assert!(words.len() == 5);
        teardown_files();
    }

    #[test]
    fn test_modify_file() {
        setup_files();
        let mut words: Vec<String> = Vec::new();
        let result = read_file(&UNIT_TEST_FILE.to_string(), &mut words);
        assert!(!result.is_err());
        remove_invalid_words(&mut words);
        assert!(words.len() == 4);
        teardown_files();
    }

    #[test]
    fn test_write_file() {
        setup_files();
        let mut words: Vec<String> = Vec::new();
        let result = read_file(&UNIT_TEST_FILE.to_string(), &mut words);
        assert!(!result.is_err());
        remove_invalid_words(&mut words);
        assert!(words.len() == 4);
        write_file(&UNIT_TEST_OUTPUT_FILE.to_string(), &words);
        // compare the file with the expected output
        let mut output_words: Vec<String> = Vec::new();
        let result = read_file(&UNIT_TEST_OUTPUT_FILE.to_string(), &mut output_words);
        assert!(!result.is_err());
        let matching = words
            .iter()
            .zip(output_words.iter())
            .filter(|&(words, output_words)| words == output_words)
            .count();
        assert!(matching == words.len() && matching == output_words.len());
        teardown_files();
    }
}
