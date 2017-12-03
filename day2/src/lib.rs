use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_file<P: AsRef<Path>>(filename: P) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|line| line
                      .split_whitespace()
                      .map(|num| num.parse().unwrap())
                      .collect()).collect()
}

pub fn process(data: &[Vec<i32>]) -> i32 {
    data.iter()
        .map(|xs| xs.iter().max().unwrap_or(&0) - xs.iter().min().unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests;
