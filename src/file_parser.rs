use std::fs;

pub fn parse_file(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(fs::read_to_string(file_path)?
        .lines()
        .map(|s| s.to_string())
        .collect())
}
