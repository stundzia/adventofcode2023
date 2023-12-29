use std::fs::File;
use std::io::{self, BufRead};

pub fn read_file_lines(file_path: &str) -> io::Result<Vec<String>> {
    // Attempt to open the file
    let file = File::open(file_path)?;

    // Create a buffered reader to read the file line by line
    let reader = io::BufReader::new(file);

    // Collect lines into a vector
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}